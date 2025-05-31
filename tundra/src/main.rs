// src/main.rs 
use std::{env, fs, rc::Rc, cell::RefCell};
#[macro_use] extern crate lazy_static;

use tundra::{
    bytecode::{
        chunk::Chunk,
        debug::disassemble_chunk,
    }, compiler::compiler::Compiler, jit::JitContext, lexer::scanner::Scanner, vm::{interpretresult::InterpretResult, vm::VM}
};

lazy_static! {

    static ref JIT_CTX: std::sync::Mutex<JitContext> =
        std::sync::Mutex::new(JitContext::new());
}

fn read_source() -> String {
    let demo = r#"
# Factorial Function in Tundra
fun fact(n):
    var ans = 1
    for i in range(1,n+1):
        ans = ans * i
    return ans

var n = input()
var x = parseInt(n)
print(fact(x))

"#;
    demo.to_string()
}

fn main() {
    let source = read_source();
    let tokens = Scanner::new(source.clone()).scan_tokens();
    for t in &tokens {
        println!("{:?}: `{}`", t.token, t.lexeme);
    }
    println!("--- end tokens ---\n");

    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    if !compiler.compile(&source) {
        eprintln!("💥  compile failed");
        std::process::exit(65);
    }
    disassemble_chunk(&*chunk.borrow(), "== Disassembled Bytecode ==");
    let mut vm = VM::new_interpreter_only(chunk);

    match vm.run() {
        InterpretResult::Ok            => {}
        InterpretResult::RuntimeError  => eprintln!("💥  runtime error"),
        InterpretResult::CompileError  => unreachable!("we already compiled"),
    }
}
