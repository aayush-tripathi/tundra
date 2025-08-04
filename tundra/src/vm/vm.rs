//src/vm/vm.rs
use super::interpretresult::InterpretResult;
use crate::{
    bytecode::{
        self,
        chunk::Chunk,
        opcode::OpCode,
        value::{FunctionObject, Value, ValueType},
    },
    compiler::compiler::Compiler,
    jit::{JittedFn, GLOBAL_NAMES},
};
use lazy_static::lazy_static;
use std::io::{self, Write};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::jit::JitContext;
use std::sync::Mutex;

lazy_static! {
    pub static ref JIT_CTX: Mutex<JitContext> = Mutex::new(JitContext::new());
}

/// Reads one line from stdin, trims newline, returns it as a Tundra string.
fn input_native(_args: &[Value]) -> Value {
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from stdin");
    let s = buf.trim_end().to_string();
    Value::string(s)
}

/// Parses a Tundra string into an integer.
fn parse_int_native(args: &[Value]) -> Value {
    if args.len() != 1 {
        panic!("parseInt expects one argument");
    }
    match &args[0].value {
        ValueType::String(s) => {
            let n = s
                .trim()
                .parse::<i64>()
                .expect(&format!("Invalid integer: `{}`", s));
            Value::int(n)
        }
        _ => panic!("parseInt expects a string argument"),
    }
}
fn parse_float_native(args: &[Value]) -> Value {
    let s = match &args[0].value {
        ValueType::String(s) => s,
        _ => panic!("parseFloat expects a string"),
    };
    let f = s
        .trim()
        .parse::<f64>()
        .expect(&format!("Invalid float: `{}`", s));
    Value::float(f)
}
fn len_native(args: &[Value]) -> Value {
    use bytecode::value::ValueType;
    assert_eq!(args.len(), 1, "len() expects one argument");
    if let ValueType::Array(rc_vec) = &args[0].value {
        let n = rc_vec.borrow().len() as i64;
        Value::int(n)
    } else {
        panic!("len() expects an array, got {:?}", args[0].value);
    }
}
/// Native implementation of `Array(len)`.
fn array_native(args: &[Value]) -> Value {
    if args.len() != 1 {
        panic!("Array() expects exactly one argument");
    }

    let len = match args[0].value {
        ValueType::Int(n) if n >= 0 => n as usize,
        _ => panic!("Array() expects a nonnegative integer"),
    };

    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(Value::none());
    }
    Value::array(v)
}

/// A call‐frame saved on CALL, popped on RETURN.
struct CallFrame {
    return_pc: usize,
    return_reg: usize,
    chunk: Rc<RefCell<Chunk>>,
    base: usize,
}
pub struct VM {
    pub registers: [Value; 1024],
    pub chunk: Rc<RefCell<Chunk>>,
    pub pc: usize,
    pub globals: HashMap<&'static str, Value>,
    pub frames: Vec<CallFrame>,
    pub jit_enabled: bool,
    output: &'static mut dyn Write,
}

impl VM {
     pub fn new(chunk: Rc<RefCell<Chunk>>, output: &'static mut dyn Write,) -> Self{
        let mut vm = VM {
            registers: core::array::from_fn(|_| Value::none()),
            chunk,
            pc: 0,
            globals: HashMap::new(),
            frames: Vec::new(),
            jit_enabled: true,
            output,
        };
        vm.globals.insert("input", Value::native(input_native, 0));
        vm.globals
            .insert("parseInt", Value::native(parse_int_native, 1));
        vm.globals
            .insert("parseFloat", Value::native(parse_float_native, 1));
        vm.globals.insert("Array", Value::native(array_native, 1));

        vm.globals.insert("len", Value::native(len_native, 1));
        vm
    }
    pub fn new_interpreter_only(chunk: Rc<RefCell<Chunk>>,output: &'static mut dyn Write,) -> Self {
        let mut vm = VM {
            registers: core::array::from_fn(|_| Value::none()),
            chunk,
            pc: 0,
            globals: HashMap::new(),
            frames: Vec::new(),
            jit_enabled: false,
            output
        };

        vm.globals.insert("input", Value::native(input_native, 0));
        vm.globals
            .insert("parseInt", Value::native(parse_int_native, 1));
        vm.globals
            .insert("parseFloat", Value::native(parse_float_native, 1));
        vm.globals.insert("Array", Value::native(array_native, 1));

        vm.globals.insert("len", Value::native(len_native, 1));
        vm.jit_enabled = false;
        vm
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        *self.chunk.borrow_mut() = Chunk::new();
        self.reset_registers();
        let mut compiler = Compiler::new(self.chunk.clone());
        if !compiler.compile(source) {
            return InterpretResult::CompileError;
        }
        self.run()
    }

    fn reset_registers(&mut self) {
        self.pc = 0;
        self.registers = core::array::from_fn(|_| Value::none());
        self.frames.clear();
    }

    pub fn add(&self, a: &Value, b: &Value) -> Value {
        use bytecode::value::ValueType::*;
        match (&a.value, &b.value) {
            (Int(x), Int(y)) => Value::int(x + y),
            (Float(x), Float(y)) => Value::float(x + y),
            (String(x), String(y)) => Value::string(x.clone() + y),
            (String(x), Int(y)) => Value::string(x.clone() + &y.to_string()),
            (String(x), Float(y)) => Value::string(x.clone() + &y.to_string()),
            (String(x), Char(y)) => Value::string(x.clone() + &y.to_string()),
            (Int(x), String(y)) => Value::string(x.to_string() + y),
            (Int(x), Float(y)) => Value::float((*x as f64) + y),
            (Float(x), String(y)) => Value::string(x.to_string() + y),
            (Float(x), Int(y)) => Value::float(x + (*y as f64)),
            (Char(x), String(y)) => Value::string(x.to_string() + y),
            (Char(x), Char(y)) => Value::string(x.to_string() + &y.to_string()),
            (Bool(x), Bool(y)) => Value::int((*x as i64) + (*y as i64)),
            (Bool(x), Int(y)) => Value::int((*x as i64) + y),
            (Int(x), Bool(y)) => Value::int(x + (*y as i64)),
            (Bool(x), Float(y)) => Value::float((*x as i64) as f64 + y),
            (Float(x), Bool(y)) => Value::float(x + (*y as i64) as f64),

            (None, Int(y)) => Value::int(*y),
            (Int(x), None) => Value::int(*x),
            (None, Float(y)) => Value::float(*y),
            (Float(x), None) => Value::float(*x),
            (None, None) => Value::int(0),
            _ => {
                eprintln!(
                    "🛑 add(): unsupported operand types: {:?} + {:?}",
                    a.value, b.value
                );
                panic!("Invalid types for Add operation");
            }
        }
    }

    fn subtract(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a - b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a - b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64) - b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a - (*b as f64)),
            _ => panic!("Invalid types for Subtract operation"),
        }
    }

    pub fn multiply(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a * b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a * b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64) * b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a * (*b as f64)),
            _ => panic!("Invalid types for Mult operation"),
        }
    }
    pub fn divide(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::float((*a as f64) / (*b as f64)),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a / b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64) / b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a / (*b as f64)),
            _ => panic!("Invalid types for Divide operation"),
        }
    }
    pub fn intdivide(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a / b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::int((a / b) as i64),
            (ValueType::Int(a), ValueType::Float(b)) => Value::int(((*a as f64) / b) as i64),
            (ValueType::Float(a), ValueType::Int(b)) => Value::int((a / (*b as f64)) as i64),
            _ => panic!("Invalid types for Floor operation"),
        }
    }
    pub fn exponentiate(&self, base: &Value, exp: &Value) -> Value {
        match (&base.value, &exp.value) {
            (ValueType::Int(b), ValueType::Int(e)) => {
                if *e < 0 {
                    panic!("Negative exponent {} for integer power", e);
                }
                let result = (*b).pow(*e as u32);
                Value::int(result)
            }
            (ValueType::Float(b), ValueType::Float(e)) => Value::float(b.powf(*e)),
            (ValueType::Float(b), ValueType::Int(e)) => Value::float(b.powf(*e as f64)),
            (ValueType::Int(b), ValueType::Float(e)) => Value::float((*b as f64).powf(*e)),
            _ => panic!("Invalid types for Exponentiate: {:?} ^ {:?}", base, exp),
        }
    }

    pub fn equal(&self, a: &Value, b: &Value) -> Value {
        use bytecode::value::ValueType::*;
        let result = match (&a.value, &b.value) {
            (Int(x), Int(y)) => x == y,
            (Float(x), Float(y)) => x == y,
            (Int(x), Float(y)) => (*x as f64) == *y,
            (Float(x), Int(y)) => *x == (*y as f64),
            (String(x), String(y)) => x == y,
            (Char(x), Char(y)) => x == y,
            (Bool(x), Bool(y)) => x == y,
            _ => false,
        };
        Value::boolean(result)
    }

    pub fn not_equal(&self, a: &Value, b: &Value) -> Value {
        let eq = self.equal(a, b);
        self.not(&eq)
    }

    pub fn greater(&self, a: &Value, b: &Value) -> Value {
        use bytecode::value::ValueType::*;
        let res = match (&a.value, &b.value) {
            (Int(x), Int(y)) => x > y,
            (Float(x), Float(y)) => x > y,
            (Int(x), Float(y)) => (*x as f64) > *y,
            (Float(x), Int(y)) => *x > (*y as f64),
            (String(x), String(y)) => x > y,
            _ => panic!("Invalid types for `>`: {:?} > {:?}", a.value, b.value),
        };
        Value::boolean(res)
    }

    pub fn less(&self, a: &Value, b: &Value) -> Value {
        use bytecode::value::ValueType::*;
        let res = match (&a.value, &b.value) {
            (Int(x), Int(y)) => x < y,
            (Float(x), Float(y)) => x < y,
            (Int(x), Float(y)) => (*x as f64) < *y,
            (Float(x), Int(y)) => *x < (*y as f64),
            (String(x), String(y)) => x < y,
            _ => panic!("Invalid types for `<`: {:?} < {:?}", a.value, b.value),
        };
        Value::boolean(res)
    }

    pub fn greater_equal(&self, a: &Value, b: &Value) -> Value {
        let gt = self.greater(a, b);
        let eq = self.equal(a, b);
        self.or(&gt, &eq)
    }

    pub fn less_equal(&self, a: &Value, b: &Value) -> Value {
        let lt = self.less(a, b);
        let eq = self.equal(a, b);
        self.or(&lt, &eq)
    }
    pub fn not(&self, a: &Value) -> Value {
        match &a.value {
            ValueType::Bool(b) => Value::boolean(!b),
            _ => panic!("Invalid type for Not operation"),
        }
    }

    pub fn or(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Bool(a), ValueType::Bool(b)) => Value::boolean(*a || *b),
            _ => panic!("Invalid types for Or operation"),
        }
    }

    fn bitwise_not(&self, value: &Value) -> Value {
        match value.value {
            ValueType::Int(v) => Value::int(!v),
            ValueType::Bool(v)=> Value::boolean(!v),
            _ => panic!("Invalid type for BitwiseNot operation"),
        }
    }

    fn negate(&self, value: &Value) -> Value {
        use bytecode::value::ValueType;
        match &value.value {
            ValueType::Int(v) => Value::int(-v),
            ValueType::Float(v) => Value::float(-v),
            ValueType::Bool(b) => Value::boolean(!b),
            other => panic!("Invalid type for Negate operation: {:?}", other),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value.value {
            ValueType::Bool(b) => b,
            ValueType::None => false,
            ValueType::Int(b) => b != 0,
            ValueType::Float(b) => b != 0.0,
            _ => true,
        }
    }
    pub fn is_falsey(&self, val: &Value) -> bool {
        match val.value {
            ValueType::None => true,
            ValueType::Bool(bool_val) => !bool_val,
            ValueType::Int(val) => val == 0,
            ValueType::Float(val) => val == 0.0,
            _ => false,
        }
    }
    pub fn bitwise_and(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a & b),
            (ValueType::Bool(a), ValueType::Bool(b)) => Value::boolean(*a & *b),
            _ => panic!("Invalid types for BitwiseAnd operation"),
        }
    }
    pub fn bitwise_or(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a | b),
            (ValueType::Bool(a), ValueType::Bool(b)) => Value::boolean(*a || *b),
            _ => panic!("Invalid types for BitwiseOr operation"),
        }
    }

    pub fn bitwise_xor(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a ^ b),
            _ => panic!("Invalid types for BitwiseXor operation"),
        }
    }

    pub fn invoke_from_jit(&mut self, base: usize, callee_slot: usize, argc: usize) -> i64 {
        let callee_val = self.registers[callee_slot].clone();

        match callee_val.value {
            ValueType::NativeFunction(func, arity) => {
                assert_eq!(arity, argc);
                let args: Vec<_> = (0..argc)
                    .map(|i| self.registers[callee_slot + 1 + i].clone())
                    .collect();
                let rv = func(&args);
                return rv.as_i64();
            }

            ValueType::Function(ref f_rc) => {
                {
                    let mut fobj = f_rc.borrow_mut();
                    if fobj.jitted.is_none() {
                        JIT_CTX.lock().unwrap().compile_function(&mut *fobj);
                    }
                }

                let code_ptr = f_rc.borrow().jitted.expect("compiled ptr");

                unsafe {
                    match argc {
                        0 => {
                            let fp: extern "C" fn(i64, i64) -> i64 = core::mem::transmute(code_ptr);
                            fp(self as *mut _ as i64, base as i64)
                        }
                        1 => {
                            let fp: extern "C" fn(i64, i64, i64) -> i64 =
                                core::mem::transmute(code_ptr);
                            fp(
                                self as *mut _ as i64,
                                base as i64,
                                self.registers[callee_slot + 1].as_i64(),
                            )
                        }
                        2 => {
                            let fp: extern "C" fn(i64, i64, i64, i64) -> i64 =
                                core::mem::transmute(code_ptr);
                            fp(
                                self as *mut _ as i64,
                                base as i64,
                                self.registers[callee_slot + 1].as_i64(),
                                self.registers[callee_slot + 2].as_i64(),
                            )
                        }

                        3 => {
                            let fp: extern "C" fn(i64, i64, i64, i64, i64) -> i64 =
                                core::mem::transmute(code_ptr);
                            fp(
                                self as *mut _ as i64,
                                base as i64,
                                self.registers[callee_slot + 1].as_i64(),
                                self.registers[callee_slot + 2].as_i64(),
                                self.registers[callee_slot + 3].as_i64(),
                            )
                        }
                        _ => todo!(">3 args"),
                    }
                }
            }

            _ => panic!("invoke_from_jit on non-function value"),
        }
    }

    /// Allocate a fresh array in a new register, return its slot.
    pub fn new_array_jit(&mut self, len: usize) -> usize {
        let slot = self
            .registers
            .iter()
            .position(|v| matches!(v.value, ValueType::None))
            .expect("no free register for new array");
        self.registers[slot] = Value::array(vec![Value::none(); len]);
        slot
    }

    pub fn array_get_jit(&mut self, arr_slot: usize, idx: usize) -> usize {
        let slot = self.new_array_jit(0); // temp slot for the value
        if let ValueType::Array(ref rc_vec) = self.registers[arr_slot].value {
            let val = rc_vec.borrow()[idx].clone();
            self.registers[slot] = val;
            slot
        } else {
            panic!("array_get_jit on non-array");
        }
    }

    pub fn array_set_jit(&mut self, arr_slot: usize, idx: usize, val_slot: usize) {
        let new_val = self.registers[val_slot].clone();
        if let ValueType::Array(ref rc_vec) = &mut self.registers[arr_slot].value {
            rc_vec.borrow_mut()[idx] = new_val;
        } else {
            panic!("array_set_jit on non-array");
        }
    }

    pub fn print_jit(&mut self, val_slot: usize) {
        writeln!(self.output, "{}", self.registers[val_slot]).unwrap();
    }
}
type OpHandler = for<'vm> fn(&'vm mut VM, OpCode);
lazy_static! {
    static ref DISPATCH_TABLE: Vec<OpHandler> = {
        let mut table: Vec<OpHandler> = Vec::with_capacity(0x27);


        table.push(handle_invalid);            // 0x00
        table.push(VM::handle_load_constant);  // 0x01
        table.push(VM::handle_define_global);  // 0x02
        table.push(VM::handle_get_global);     // 0x03
        table.push(VM::handle_set_global);     // 0x04
        table.push(VM::handle_get_local);      // 0x05
        table.push(VM::handle_set_local);      // 0x06
        table.push(VM::handle_jump_if_false);  // 0x07
        table.push(VM::handle_jump);           // 0x08
        table.push(VM::handle_loop);           // 0x09
        table.push(VM::handle_move);           // 0x0A
        table.push(VM::handle_true);           // 0x0B
        table.push(VM::handle_false);          // 0x0C
        table.push(VM::handle_pop);            // 0x0D
        table.push(VM::handle_none);           // 0x0E
        table.push(VM::handle_add);            // 0x0F
        table.push(VM::handle_subtract);       // 0x10
        table.push(VM::handle_multiply);       // 0x11
        table.push(VM::handle_divide);         // 0x12
        table.push(VM::handle_exponentiate);   // 0x13
        table.push(VM::handle_int_divide);     // 0x14
        table.push(VM::handle_return);         // 0x15
        table.push(VM::handle_bitwise_not);    // 0x16
        table.push(VM::handle_negate);         // 0x17
        table.push(VM::handle_print);          // 0x18
        table.push(VM::handle_equal);          // 0x19
        table.push(VM::handle_greater);        // 0x1A
        table.push(VM::handle_less);           // 0x1B
        table.push(VM::handle_bitwise_and);    // 0x1C
        table.push(VM::handle_bitwise_or);     // 0x1D
        table.push(VM::handle_bitwise_xor);    // 0x1E
        table.push(VM::handle_greater_equal);  // 0x1F
        table.push(VM::handle_less_equal);     // 0x20
        table.push(VM::handle_not_equal);      // 0x21
        table.push(VM::handle_call);           // 0x22
        table.push(VM::handle_mod);            // 0x23
        table.push(VM::handle_new_array);      // 0x24
        table.push(VM::handle_get_index);      // 0x25
        table.push(VM::handle_set_index);      // 0x26
        table.push(VM::handle_inc_loop_if_less); // 0x27

        table
    };
}
fn handle_invalid(_vm: &mut VM, _op: OpCode) {
    panic!("Invalid opcode tag 0 encountered");
}

impl VM{
    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instr = {
                let chunk = self.chunk.borrow();
                if self.pc >= chunk.code.len() {
                    return InterpretResult::Ok;
                }
                chunk.code[self.pc].clone()
            };
            self.pc += 1;
            let tag = instr.tag() as usize;
            (DISPATCH_TABLE[tag])(self, instr);
        }
    }

    fn base(&self) -> usize {
        self.frames.last().map(|f| f.base).unwrap_or(0)
    }

    pub fn handle_load_constant(&mut self, op: OpCode) {
        if let OpCode::LoadConstant(dest, val) = op {
            let b = self.base();
            self.registers[b + dest] = val;
        } else {
            unreachable!()
        }
    }

    pub fn handle_inc_loop_if_less(&mut self, op: OpCode) {
        if let OpCode::IncLoopIfLess(idx_r, limit_r, target) = op {
            let b = self.base();
            if self
                .less(&self.registers[b + idx_r], &self.registers[b + limit_r])
                .value
                == ValueType::Bool(true)
            {
                // idx += 1
                let one = Value::int(1);
                self.registers[b + idx_r] = self.add(&self.registers[b + idx_r], &one);
                self.pc = target;
            }
        }
    }

    pub fn handle_define_global(&mut self, op: OpCode) {
        if let OpCode::DefineGlobal(r, name) = op {
            let b = self.base();

            let table = GLOBAL_NAMES.lock().unwrap();
            let (leaked_str, _) = table
                .get_key_value(name.as_str())
                .expect("global not interned");

            self.globals
                .insert(*leaked_str, self.registers[b + r].clone());
        } else {
            unreachable!()
        }
    }

    pub fn handle_get_global(&mut self, op: OpCode) {
        if let OpCode::GetGlobal(d, name) = op {
            let b = self.base();
            let key: &str = name.as_str();
            let val = self
                .globals
                .get(key)
                .unwrap_or_else(|| panic!("Undefined global '{}'", key))
                .clone();

            self.registers[b + d] = val;
        } else {
            unreachable!()
        }
    }

    pub fn handle_set_global(&mut self, op: OpCode) {
        if let OpCode::SetGlobal(r, name) = op {
            let b = self.base();
            let table = GLOBAL_NAMES.lock().unwrap();
            let (leaked_str, _) = table
                .get_key_value(name.as_str())
                .expect("global not interned");

            if !self.globals.contains_key(*leaked_str) {
                panic!("Undefined global '{}'", leaked_str);
            }

            self.globals
                .insert(*leaked_str, self.registers[b + r].clone());
        } else {
            unreachable!()
        }
    }

    pub fn handle_get_local(&mut self, op: OpCode) {
        if let OpCode::GetLocal(dst, slot) = op {
            let b = self.base();
            self.registers[b + dst] = self.registers[b + slot].clone();
        } else {
            unreachable!()
        }
    }

    pub fn handle_set_local(&mut self, op: OpCode) {
        if let OpCode::SetLocal(src, slot) = op {
            let b = self.base();
            self.registers[b + slot] = self.registers[b + src].clone();
        } else {
            unreachable!()
        }
    }

    pub fn handle_jump_if_false(&mut self, op: OpCode) {
        if let OpCode::JumpIfFalse(r, offset) = op {
            let b = self.base();
            if !self.is_truthy(&self.registers[b + r]) {
                self.pc = self.pc.wrapping_add(offset);
            }
        } else {
            unreachable!()
        }
    }

    pub fn handle_jump(&mut self, op: OpCode) {
        if let OpCode::Jump(offset) = op {
            self.pc = self.pc.wrapping_add(offset);
        } else {
            unreachable!()
        }
    }

    pub fn handle_loop(&mut self, op: OpCode) {
        if let OpCode::Loop(offset) = op {
            self.pc = self.pc.wrapping_sub(offset);
        } else {
            unreachable!()
        }
    }

    pub fn handle_move(&mut self, op: OpCode) {
        if let OpCode::Move(d, s) = op {
            let b = self.base();
            self.registers[b + d] = self.registers[b + s].clone();
        } else {
            unreachable!()
        }
    }

    pub fn handle_true(&mut self, op: OpCode) {
        if let OpCode::True(d) = op {
            let b = self.base();
            self.registers[b + d] = Value::boolean(true);
        } else {
            unreachable!()
        }
    }

    pub fn handle_false(&mut self, op: OpCode) {
        if let OpCode::False(d) = op {
            let b = self.base();
            self.registers[b + d] = Value::boolean(false);
        } else {
            unreachable!()
        }
    }

    pub fn handle_none(&mut self, op: OpCode) {
        if let OpCode::None(d) = op {
            let b = self.base();
            self.registers[b + d] = Value::none();
        } else {
            unreachable!()
        }
    }

    pub fn handle_pop(&mut self, op: OpCode) {
        if let OpCode::Pop(d) = op {
            let b = self.base();
            self.registers[b + d] = Value::none();
        } else {
            unreachable!()
        }
    }

    pub fn handle_add(&mut self, op: OpCode) {
        if let OpCode::Add(d, a, b2) = op {
            let b = self.base();
            self.registers[b + d] = self.add(&self.registers[b + a], &self.registers[b + b2]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_subtract(&mut self, op: OpCode) {
        if let OpCode::Subtract(d, a, b2) = op {
            let b = self.base();
            self.registers[b + d] = self.subtract(&self.registers[b + a], &self.registers[b + b2]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_multiply(&mut self, op: OpCode) {
        if let OpCode::Multiply(d, a, b2) = op {
            let b = self.base();
            self.registers[b + d] = self.multiply(&self.registers[b + a], &self.registers[b + b2]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_divide(&mut self, op: OpCode) {
        if let OpCode::Divide(d, a, b2) = op {
            let b = self.base();
            self.registers[b + d] = self.divide(&self.registers[b + a], &self.registers[b + b2]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_exponentiate(&mut self, op: OpCode) {
        if let OpCode::Exponentiate(d, e, base_r) = op {
            let b = self.base();
            self.registers[b + d] =
                self.exponentiate(&self.registers[b + base_r], &self.registers[b + e]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_int_divide(&mut self, op: OpCode) {
        if let OpCode::IntDivide(d, a, b2) = op {
            let b = self.base();
            self.registers[b + d] = self.intdivide(&self.registers[b + a], &self.registers[b + b2]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_mod(&mut self, op: OpCode) {
        if let OpCode::Mod(d, a, bb) = op {
            let b = self.base();
            let va = &self.registers[b + a];
            let vb = &self.registers[b + bb];
            match (&va.value, &vb.value) {
                (ValueType::Int(x), ValueType::Int(y)) => self.registers[b + d] = Value::int(x % y),
                _ => panic!("Invalid types for `%`: {:?} % {:?}", va, vb),
            }
        } else {
            unreachable!()
        }
    }

    pub fn handle_return(&mut self, op: OpCode) {
        if let OpCode::Return(r) = op {
            let b = self.base();
            let rv = self.registers[b + r].clone();

            match self.frames.pop() {
                Some(frame) => {
                    self.chunk = frame.chunk;
                    self.pc = frame.return_pc;
                    self.registers[frame.return_reg] = rv;
                }

                None => {
                    self.pc = self.chunk.borrow().code.len();
                }
            }
        }
    }

    pub fn handle_bitwise_not(&mut self, op: OpCode) {
        if let OpCode::BitwiseNot(d, s) = op {
            let b = self.base();
            self.registers[b + d] = self.bitwise_not(&self.registers[b + s]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_negate(&mut self, op: OpCode) {
        if let OpCode::Negate(d, s) = op {
            let b = self.base();
            self.registers[b + d] = self.negate(&self.registers[b + s]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_print(&mut self, op: OpCode) {
        if let OpCode::Print(r) = op {
            let b = self.base();
            writeln!(self.output, "{}", self.registers[b + r]).unwrap();
        } else {
            unreachable!()
        }
    }

    pub fn handle_equal(&mut self, op: OpCode) {
        if let OpCode::Equal(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] = self.equal(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_not_equal(&mut self, op: OpCode) {
        if let OpCode::NotEqual(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] = self.not_equal(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_greater(&mut self, op: OpCode) {
        if let OpCode::Greater(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] = self.greater(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_less(&mut self, op: OpCode) {
        if let OpCode::Less(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] = self.less(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_bitwise_and(&mut self, op: OpCode) {
        if let OpCode::BitwiseAnd(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] =
                self.bitwise_and(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_bitwise_or(&mut self, op: OpCode) {
        if let OpCode::BitwiseOr(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] =
                self.bitwise_or(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_bitwise_xor(&mut self, op: OpCode) {
        if let OpCode::BitwiseXor(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] =
                self.bitwise_xor(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_greater_equal(&mut self, op: OpCode) {
        if let OpCode::GreaterEqual(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] =
                self.greater_equal(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_less_equal(&mut self, op: OpCode) {
        if let OpCode::LessEqual(d, a, bb) = op {
            let b = self.base();
            self.registers[b + d] =
                self.less_equal(&self.registers[b + a], &self.registers[b + bb]);
        } else {
            unreachable!()
        }
    }
    fn call_interpreted(
        &mut self,
        f_rc: Rc<RefCell<FunctionObject>>,
        dest: usize,
        argc: usize,
        callee_reg: usize,
    ) {
        let caller_base = self.base();
        let callee_slot = caller_base + callee_reg;
        let new_base = callee_slot + 1;

        self.frames.push(CallFrame {
            return_pc: self.pc,
            return_reg: caller_base + dest,
            chunk: self.chunk.clone(),
            base: new_base,
        });
        self.chunk = f_rc.borrow().chunk.clone();
        self.pc = 0;
    }
    pub fn handle_call(&mut self, op: OpCode) {
        if let OpCode::Call(dest, callee_reg, argc) = op {
            let frame_base = self.base();
            let callee_slot = frame_base + callee_reg;
            let mut arg_buf = Vec::<i64>::with_capacity(argc);
            for i in 0..argc {
                arg_buf.push(self.registers[callee_slot + 1 + i].as_i64());
            }

            let callee_val = self.registers[callee_slot].clone();
            let rv = match callee_val.value {
                ValueType::Function(ref f_rc) => {
                    if !self.jit_enabled {
                        self.call_interpreted(f_rc.clone(), dest, argc, callee_reg);
                        return;
                    }
                    {
                        self.frames.push(CallFrame {
                            return_pc: self.pc,
                            return_reg: frame_base + dest,
                            chunk: self.chunk.clone(),
                            base: frame_base + callee_reg,
                        });
                        let mut fobj = f_rc.borrow_mut();
                        if fobj.jitted.is_none() {
                            JIT_CTX.lock().unwrap().compile_function(&mut *fobj);
                        }
                    }

                    let code_ptr = f_rc.borrow().jitted.expect("compiled ptr");
                    let frame_base = self.base() as i64;
                    let self_ptr = self as *mut _ as i64;

                    unsafe {
                        match argc {
                            0 => {
                                let fp: extern "C" fn(i64, i64) -> i64 =
                                    std::mem::transmute(code_ptr);
                                fp(self_ptr, frame_base)
                            }
                            1 => {
                                let fp: extern "C" fn(i64, i64, i64) -> i64 =
                                    std::mem::transmute(code_ptr);
                                fp(self_ptr, frame_base, arg_buf[0])
                            }
                            2 => {
                                let fp: extern "C" fn(i64, i64, i64, i64) -> i64 =
                                    std::mem::transmute(code_ptr);
                                fp(self_ptr, frame_base, arg_buf[0], arg_buf[1])
                            }
                            3 => {
                                let fp: extern "C" fn(i64, i64, i64, i64, i64) -> i64 =
                                    std::mem::transmute(code_ptr);
                                fp(self_ptr, frame_base, arg_buf[0], arg_buf[1], arg_buf[2])
                            }
                            n => {
                                extern "C" {
                                    fn tundra_apply_variadic(
                                        fn_ptr: *const u8,
                                        vm_ptr: i64,
                                        base: i64,
                                        argv: *const i64,
                                        argc: i64,
                                    ) -> i64;
                                }
                                tundra_apply_variadic(
                                    code_ptr,
                                    self_ptr,
                                    frame_base,
                                    arg_buf.as_ptr(),
                                    n as i64,
                                )
                            }
                        }
                    }
                }
                ValueType::NativeFunction(func, arity) => {
                    assert_eq!(arity, argc);
                    let args: Vec<_> = (0..argc)
                        .map(|i| self.registers[callee_slot + 1 + i].clone())
                        .collect();
                    let v = func(&args);
                    self.registers[frame_base + dest] = v;
                    return; // done
                }

                _ => panic!("attempt to call a non-function value"),
            };

            let rv_val = unsafe { Value::from_i64(rv) };

            let frame = self.frames.pop().unwrap();
            self.chunk = frame.chunk;
            self.pc = frame.return_pc;
            self.registers[frame.return_reg] = rv_val;
        }
    }

    pub fn handle_new_array(&mut self, op: OpCode) {
        if let OpCode::NewArray(dest, len_reg) = op {
            let b = self.base();
            let len = match self.registers[b + len_reg].value {
                ValueType::Int(n) if n >= 0 => n as usize,
                _ => panic!("Invalid array length"),
            };
            self.registers[b + dest] = Value::array(vec![Value::none(); len]);
        } else {
            unreachable!()
        }
    }

    pub fn handle_get_index(&mut self, op: OpCode) {
        if let OpCode::GetIndex(dest, arr_r, idx_r) = op {
            let b = self.base();
            let idx = match self.registers[b + idx_r].value {
                ValueType::Int(i) => i as usize,
                _ => panic!("Non‐int index"),
            };
            if let ValueType::Array(ref rc_vec) = self.registers[b + arr_r].value {
                let element = {
                    let vec_ref = rc_vec.borrow();
                    vec_ref[idx].clone()
                };
                self.registers[b + dest] = element;
            } else {
                panic!("Indexing non‐array");
            }
        } else {
            unreachable!()
        }
    }

    pub fn handle_set_index(&mut self, op: OpCode) {
        if let OpCode::SetIndex(arr_r, idx_r, val_r) = op {
            let b = self.base();
            let idx = match self.registers[b + idx_r].value {
                ValueType::Int(i) => i as usize,
                _ => panic!("Non‐int index"),
            };
            let new_val = self.registers[b + val_r].clone();
            if let ValueType::Array(ref rc_vec) = &mut self.registers[b + arr_r].value {
                let mut vec_ref = rc_vec.borrow_mut();
                vec_ref[idx] = new_val;
            } else {
                panic!("Index assignment on non‐array");
            }
        } else {
            unreachable!()
        }
    }
}
