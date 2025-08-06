// src/main.rs
use std::{cell::RefCell, env, fs, io, rc::Rc};
#[macro_use]
extern crate lazy_static;

use tundra::{
    bytecode::{chunk::Chunk, debug::disassemble_chunk},
    compiler::compiler::Compiler,
    jit::JitContext,
    lexer::scanner::Scanner,
    vm::{interpretresult::InterpretResult, vm::VM},
};

lazy_static! {
    static ref JIT_CTX: std::sync::Mutex<JitContext> = std::sync::Mutex::new(JitContext::new());
}

fn read_source() -> String {
    let demo = r#"
var a = [4,7,10,11,-1,5]
fun bubbleSort(a):
    var n = 6
    for i in range(n):
        for j in range(n-i-1):
            if (a[j]>a[j+1]):
                var temp= a[j+1]
                a[j+1]=a[j]
                a[j]=temp

bubbleSort(a)
for i in range(6):
    print(a[i])

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
    let mut vm = VM::new_interpreter_only(chunk, Box::leak(Box::new(std::io::stdout())));

    match vm.run() {
        InterpretResult::Ok => {}
        InterpretResult::RuntimeError => eprintln!("💥  runtime error"),
        InterpretResult::CompileError => unreachable!("we already compiled"),
    }
}
