use std::collections::HashMap;

use crate::bytecode::{chunk::Chunk, opcode::OpCode, value::{Value, ValueType}};

use super::interpretresult::InterpretResult;

pub struct VM {
    pub registers: [Value; 128],
    pub chunk: Chunk,
    pub ip: usize,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            registers: core::array::from_fn(|_| Value::none()), // Initialize with default values
            chunk,
            ip: 0,
            globals: HashMap::new(),
        }
    }
    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        self.chunk = Chunk::new_chunk();
        self.reset_registers();
        //let mut compiler = Compiler::new(&mut self.chunk);

        //if !compiler.compile(&source) {
        //    return InterpretResult::CompileError;
       // }

        self.run()
    }
    pub fn run(&mut self)-> InterpretResult  {
        loop {
            if self.ip >= self.chunk.code.len() {
                return InterpretResult::Ok;
            }
            let instruction = &self.chunk.code[self.ip];
            self.ip += 1;

            match instruction {
                OpCode::LoadConstant(dest, constant_value) => {
                    self.registers[*dest] = constant_value.clone();
                }
                OpCode::Add(dest, src1, src2) => {
                    let result = self.add(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Subtract(dest, src1, src2) => {
                    let result = self.subtract(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Multiply(dest, src1, src2) => {
                    let result = self.multiply(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Divide(dest, src1, src2) => {
                    let result = self.divide(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Exponentiate(dest,src1, src2, )=>{
                    let result = self.exponentiate(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::IntDivide(dest, src1, src2)=>{
                    let result = self.intdivide(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Print(reg) => {
                    println!("{}", self.registers[*reg]);
                }
                OpCode::Equal(dest, src1, src2) => {
                    let result = self.equal(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::NotEqual(dest, src1, src2) => {
                    let result = self.not_equal(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Greater(dest, src1, src2) => {
                    let result = self.greater(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::LessEqual(dest, src1, src2) => {
                    let result = self.less_equal(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::Less(dest, src1, src2) => {
                    let result = self.less(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::GreaterEqual(dest, src1, src2) => {
                    let result = self.greater_equal(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::BitwiseAnd(dest, src1, src2) => {
                    let result = self.bitwise_and(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::BitwiseOr(dest, src1, src2) => {
                    let result = self.bitwise_or(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                OpCode::BitwiseXor(dest, src1, src2) => {
                    let result = self.bitwise_xor(&self.registers[*src1], &self.registers[*src2]);
                    self.registers[*dest] = result;
                }
                
                OpCode::Return(src) => {
                    println!("Returning: {}", self.registers[*src]);
                    
                }
                OpCode::BitwiseNot(dest, src) => {
                    let result = self.bitwise_not(&self.registers[*src]);
                    self.registers[*dest] = result;
                }
                OpCode::Negate(dest, src) => {
                    let result = self.negate(&self.registers[*src]);
                    self.registers[*dest] = result;
                }
                OpCode::DefineGlobal(reg, name) => {
                    let value = self.registers[*reg].clone();
                    self.globals.insert(name.clone(), value);
                }
                OpCode::GetGlobal(dest, name) => {
                    if let Some(value) = self.globals.get(name) {
                        self.registers[*dest] = value.clone();
                    } else {
                        panic!("Undefined global variable '{}'", name);
                    }
                }
                OpCode::SetGlobal(src, name) => {
                    let value = self.registers[*src].clone();
                    if self.globals.contains_key(name) {
                        self.globals.insert(name.clone(), value);
                    } else {
                        panic!("Undefined global variable '{}'", name);
                    }
                }
                OpCode::GetLocal(dest, src) => {
                    self.registers[*dest] = self.registers[*src].clone();
                }
                OpCode::SetLocal(dest, src) => {
                    self.registers[*dest] = self.registers[*src].clone();
                }
                OpCode::JumpIfFalse(reg, offset) => {
                    if !self.is_truthy(&self.registers[*reg]) {
                        self.ip += *offset;
                    }
                }
                OpCode::Jump(offset) => {
                    self.ip += *offset;
                }
                OpCode::Loop(offset) => {
                    self.ip -= *offset;
                }
                OpCode::Move(dest, src) => {
                    self.registers[*dest] = self.registers[*src].clone();
                }
                OpCode::True(dest) => {
                    self.registers[*dest] = Value::boolean(true);
                }
                OpCode::False(dest) => {
                    self.registers[*dest] = Value::boolean(false);
                }
                OpCode::Pop(dest) => {
                    self.registers[*dest] = Value::none();
                }
                OpCode::None(dest) => {
                    self.registers[*dest] = Value::none();
                }
                _ => {
                    return InterpretResult::RuntimeError;
                },
            }

        }
    }
    pub fn add(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a + b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a + b),
            (ValueType::String(a), ValueType::String(b)) => Value::string(a.clone() + b),
            (ValueType::String(a), ValueType::Int(b)) => Value::string(a.clone() + &b.to_string()),
            (ValueType::String(a), ValueType::Float(b)) => Value::string(a.clone() + &b.to_string()),
            (ValueType::String(a), ValueType::Char(b)) => Value::string(a.clone() + &b.to_string()),
            (ValueType::Int(a), ValueType::String(b)) => Value::string(a.to_string() + b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64) + b),
            (ValueType::Float(a), ValueType::String(b)) => Value::string(a.to_string() + b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a + (*b as f64)),
            (ValueType::Char(a), ValueType::String(b)) => Value::string(a.to_string() + b),
            (ValueType::Char(a), ValueType::Char(b)) => Value::string(a.to_string() + &b.to_string()),
            _ => panic!("Invalid types for Add operation"),
        }
    }

    fn subtract(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a + b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a + b),
            (ValueType::Int(a), ValueType::String(b)) => Value::string(a.to_string() + b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64) + b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a + (*b as f64)),
            _ => panic!("Invalid types for Add operation"),
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
            _ => panic!("Invalid types for Mult operation"),
        }
    }
    pub fn intdivide(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a / b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::int((a / b) as i64),
            (ValueType::Int(a), ValueType::Float(b)) => Value::int(((*a as f64) / b) as i64),
            (ValueType::Float(a), ValueType::Int(b)) => Value::int((a / (*b as f64)) as i64),
            _ => panic!("Invalid types for Mult operation"),
        }
    }
    pub fn exponentiate(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int((*a as f64).powf(*b as f64) as i64),
            (ValueType::Float(a), ValueType::Float(b)) => Value::float(a.powf(*b) ),
            (ValueType::Int(a), ValueType::Float(b)) => Value::float((*a as f64).powf(*b) as f64),
            (ValueType::Float(a), ValueType::Int(b)) => Value::float(a.powf((*b) as f64)),
            _ => panic!("Invalid types for Mult operation"),
        }
    }
    pub fn equal(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::boolean(*a == *b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::boolean(*a== *b),
            (ValueType::String(a), ValueType::String(b)) => Value::boolean(a.to_string() == b.to_string()),
            (ValueType::Bool(a), ValueType::Bool(b)) => Value::boolean(a == b),
            (ValueType::String(a), ValueType::Char(b)) => Value::boolean(*a == b.to_string()),
            (ValueType::Int(a), ValueType::Float(b)) => Value::boolean(*a as f64 == *b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::boolean(*a==*b as f64),
            (ValueType::Char(a), ValueType::Char(b)) => Value::boolean(a==b),
            _ => panic!("Invalid type to compare")   ,
         
        }
    }

    pub fn greater(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::boolean(a > b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::boolean(a > b),
            (ValueType::String(a), ValueType::String(b)) => Value::boolean(a > b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::boolean(*a as f64 > *b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::boolean(*a>*b as f64),
            _ => panic!("Invalid types for Greater operation"),
        }
    }

    pub fn less(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::boolean(a < b),
            (ValueType::Float(a), ValueType::Float(b)) => Value::boolean(a < b),
            (ValueType::String(a), ValueType::String(b)) => Value::boolean(a < b),
            (ValueType::Int(a), ValueType::Float(b)) => Value::boolean((*a as f64) < *b),
            (ValueType::Float(a), ValueType::Int(b)) => Value::boolean(*a<*b as f64),
            _ => panic!("Invalid types for Less operation"),
        }
    }

    pub fn not_equal(&self, a: &Value, b: &Value) -> Value {
        let equal = self.equal(a, b);
        self.not(&equal)
    }

    pub fn greater_equal(&self, a: &Value, b: &Value) -> Value {
        let greater = self.greater(a, b);
        let equal = self.equal(a, b);
        self.or(&greater, &equal)
    }

    pub fn less_equal(&self, a: &Value, b: &Value) -> Value {
        let less = self.less(a, b);
        let equal = self.equal(a, b);
        self.or(&less, &equal)
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
            _ => panic!("Invalid type for BitwiseNot operation"),
        }
    }

    fn negate(&self, value: &Value) -> Value {
        match value.value {
            ValueType::Int(v) => Value::int(-v),
            ValueType::Float(v) => Value::float(-v),
            _ => panic!("Invalid type for Negate operation"),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value.value {
            ValueType::Bool(b) => b,
            ValueType::None => false,
            ValueType::Int(b)=>b!=0,
            ValueType::Float(b)=>b!=0.0,
            _ => true,
        }
    }
    pub fn is_falsey(&self,val: &Value) -> bool {
        match val.value {
            ValueType::None => true,
            ValueType::Bool(bool_val) => !bool_val,
            ValueType::Int(val) => val == 0,
            ValueType::Float(val)=>val==0.0,
            _ => false,
        }
    }
    pub fn bitwise_and(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a & b),
            _ => panic!("Invalid types for BitwiseAnd operation"),
        }
    }
    pub fn bitwise_or(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a | b),
            _ => panic!("Invalid types for BitwiseOr operation"),
        }
    }

    pub fn bitwise_xor(&self, a: &Value, b: &Value) -> Value {
        match (&a.value, &b.value) {
            (ValueType::Int(a), ValueType::Int(b)) => Value::int(a ^ b),
            _ => panic!("Invalid types for BitwiseXor operation"),
        }
    }
    fn reset_registers(&mut self) {
        self.ip = 0;
        self.registers=core::array::from_fn(|_| Value::none())
    }
}


  
