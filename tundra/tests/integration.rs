// tests/integration.rs
//TO DO : REWRITE THESE TESTS
use std::{rc::Rc, cell::RefCell, io::{self, Write}};
use tundra::bytecode::{chunk::Chunk, debug::disassemble_chunk, opcode::OpCode, value::Value};
use tundra::lexer::{scanner::Scanner, token::TokenType};
use tundra::compiler::compiler::Compiler;
use tundra::vm::vm::VM;
use tundra::vm::interpretresult::InterpretResult;
#[test]
fn test_lexer_basic_tokens() {
    let src = r#"
var x = 42
print(x)
"#;
    let tokens = Scanner::new(src.to_string()).scan_tokens();
    let kinds: Vec<_> = tokens.iter().map(|t| t.token).collect();
    let expected = [
        TokenType::Newline,
        TokenType::Var, TokenType::Identifier, TokenType::Equal,
        TokenType::Int, TokenType::Newline,
        TokenType::Print, TokenType::LeftParen, TokenType::Identifier,
        TokenType::RightParen, TokenType::Newline,
        TokenType::EOF,
    ];
    assert!(kinds.windows(expected.len()).any(|w| w == expected));
}

#[test]
fn test_compile_expression() {
    let src = "var a = 1 + 2\n";
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));
    let code = &chunk.borrow().code;
    assert_eq!(code.len(), 4);
    assert!(matches!(code[0], OpCode::LoadConstant(_, _)));
    assert!(matches!(code[1], OpCode::LoadConstant(_, _)));
    assert!(matches!(code[2], OpCode::Add(_, _, _)));
    assert!(matches!(code[3], OpCode::Return(_)));
}

#[test]
fn test_vm_run_simple() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::LoadConstant(0, Value::int(2)), 1);
    chunk.write(OpCode::LoadConstant(1, Value::int(5)), 2);
    chunk.write(OpCode::Exponentiate(2, 1, 0), 3);
    chunk.write(OpCode::Print(2), 4);
    chunk.write(OpCode::Return(2), 5);

    let mut vm = VM::new(Rc::new(RefCell::new(chunk)));
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    // the exponentiation result lives in register 2
    assert_eq!(vm.registers[2], Value::int(32));
}
#[test]
fn test_full_compile_and_run() {
    let src = r#"
var x = 3
var y = 4
var x=x+y
print(x)
"#;
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    // optional disassemble
    disassemble_chunk(&chunk.borrow(), "test_full_compile");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    // find the highest‐numbered register that got set
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    assert_eq!(vm.registers[last], Value::int(7));
}
#[test]
fn test_exponentiation() {
    let src = "print(2 ** 5)\n";
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2**5=32
    assert_eq!(vm.registers[last], Value::int(32));
}

#[test]
fn test_int_division() {
    let src = "print(7 // 2)\n";
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    assert_eq!(vm.registers[last], Value::int(3));
}

#[test]
fn test_float_division() {
    let src = "print(7 / 2)\n";
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    assert_eq!(vm.registers[last], Value::float(3.5));
}

#[test]
fn test_bitwise_and_or_xor() {
    let chunk = Rc::new(RefCell::new(Chunk::new()));

    // and
    let src = "print(5 & 3)\n";
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2*5+1 = 11
    assert_eq!(vm.registers[last], Value::int(1));
    // or
    let src = "print(5 | 2)\n";
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2*5+1 = 11
    assert_eq!(vm.registers[last], Value::int(7));

    // xor
    let src = "print(5 ^ 3)\n";
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2*5+1 = 11
    assert_eq!(vm.registers[last], Value::int(6));
}

#[test]
fn test_operator_preceences() {
    // exponent has higher precedence than plus
    let src = "print(2 + 3 ** 2)\n";   // 2 + (3**2) = 11
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2*5+1 = 11
    assert_eq!(vm.registers[last], Value::int(11));

    // multiplication has higher precedence than addition
    let src2 = "print(1 + 2 * 3)\n";    // 1 + (2*3) = 7
      let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    assert!(compiler.compile(src2));

    disassemble_chunk(&chunk.borrow(), "test_some_arithmetic");

    let mut vm = VM::new(chunk.clone());
    let result = vm.run();
    assert!(matches!(result, InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .unwrap();
    // The expected result is 2*5+1 = 11
    assert_eq!(vm.registers[last], Value::int(7));
}

#[test]
fn test_for_loop_runs() {
    let src = r#"
for i in (3):
    print(i)
"#;
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    // should compile
    assert!(compiler.compile(src), "Failed to compile for-loop");
    let mut vm = VM::new(chunk.clone());
    // should run to completion without error
    assert_eq!(vm.run(), InterpretResult::Ok);
}
/// Helper to compile & run a small program and return the last non-none register.
fn run_and_get_last(src: &str) -> Value {
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut comp = Compiler::new(chunk.clone());
    assert!(comp.compile(src), "compile failed:\n{}", src);
    let mut vm = VM::new(chunk.clone());
    assert!(matches!(vm.run(), InterpretResult::Ok));
    let last = vm.registers.iter()
                  .rposition(|v| *v != Value::none())
                  .expect("no value in registers");
    vm.registers[last].clone()
}

#[test]
fn test_recursive_factorial_integration() {
    let src = r#"
fun factorial(n):
    if (n <= 1):
        return 1
    else:
        return n * factorial(n - 1)

print(factorial(6))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::int(720));
}

#[test]
fn test_sum_up_to_integration() {
    let src = r#"
fun sum_up_to(n):
    var total = 0
    for i in range(1, n + 1):
        total = total + i
    return total

print(sum_up_to(100))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::int(5050));
}

#[test]
fn test_is_prime_true_integration() {
    let src = r#"fun is_prime(n):
    if (n <= 1): return false
    for i in range(2, n):
        if (n % i == 0): return false
    return true

print(is_prime(17))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_is_prime_false_integration() {
    let src = r#"fun is_prime(n):
    if (n <= 1): return false
    for i in range(2, n):
        if (n % i == 0): return false
    return true

print(is_prime(18))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_list_primes_integration() {
    let src = r#"fun list_primes(n):
    var count = 0
    for i in range(2, n + 1):
        if (is_prime(i)):
            count = count + 1
    return count

fun is_prime(n):
    if (n <= 1): return false
    for i in range(2, n):
        if (n % i == 0): return false
    return true

print(list_primes(30))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::int(10)); // primes up to 30: 2,3,5,7,11,13,17,19,23,29
}

#[test]
fn test_string_concatenation_integration() {
    let src = r#"
print("hello, " + "world!")
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::string("hello, world!".into()));
}

#[test]
fn test_bitwise_not_integration() {
    let src = r#"
print(~6)
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::int(!6));
}

#[test]
fn test_char_and_string_concat_integration() {
    let src = r#"
print("A" + '!' )
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::string("A!".into()));
}

#[test]
fn test_nested_function_integration() {
    let src = r#"
fun outer(x):
    fun inner(y):
        return x + y
    return inner(5)

print(outer(7))
"#;
    let result = run_and_get_last(src);
    assert_eq!(result, Value::int(12));
}

#[test]
fn test_variable_shadowing_integration() {
    let src = r#"
var x = 10
fun shadow():
    var x = 3
    return x * 2
print(shadow())
print(x)
"#;
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut c = Compiler::new(chunk.clone());
    assert!(c.compile(src));
    let mut vm = VM::new(chunk.clone());
    assert!(matches!(vm.run(), InterpretResult::Ok));
    // expect first print = 6, second = 10
    let mut_seen = vm.registers.iter()
        .filter(|v| **v != Value::none())
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(mut_seen[mut_seen.len()-2], Value::int(6));
    assert_eq!(mut_seen[mut_seen.len()-1], Value::int(10));
}