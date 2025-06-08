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
var n = 1000 
var a = 0 
var b = 1
for i in range(n):
    var temp = a
    a = (a+b)%(10**9+7)
    b = temp 
print(a)


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
