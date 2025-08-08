// src/lib.rs
pub mod bytecode;
pub mod compiler;
pub mod jit;
pub mod lexer;
pub mod vm;

pub use crate::vm::interpretresult::InterpretResult;

use crate::{
    bytecode::chunk::Chunk,
    bytecode::debug::disassemble_to_string, 
    compiler::compiler::Compiler,
    lexer::scanner::Scanner,
    vm::vm::VM,
};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

struct OutputCapture {
    buffer: Rc<RefCell<Vec<u8>>>,
}
impl OutputCapture {
    fn new(buffer: Rc<RefCell<Vec<u8>>>) -> Self {
        Self { buffer }
    }
}
impl std::io::Write for OutputCapture {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn compile_only(src: &str) -> Result<Rc<RefCell<Chunk>>> {
    let _tokens = Scanner::new(src.to_string()).scan_tokens();
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    if !compiler.compile(src) {
        return Err(anyhow!("compile error"));
    }
    Ok(chunk)
}

pub fn disassemble(src: &str) -> Result<String> {
    let chunk = compile_only(src)?;
    let s = disassemble_to_string(&chunk.borrow(), "Disassembled Bytecode");
    Ok(s)
}

pub fn run(src: &str) -> Result<String> {
    // capture stdout
    let output_buffer = Rc::new(RefCell::new(Vec::<u8>::new()));
    let output_capture = Box::new(OutputCapture::new(output_buffer.clone()));
    let output_ref: &'static mut dyn std::io::Write = Box::leak(output_capture);
    let chunk = compile_only(src)?;
    // run
    let result = {
        let mut vm = VM::new_interpreter_only(chunk, output_ref);
        match vm.run() {
            InterpretResult::Ok => Ok(()),
            InterpretResult::RuntimeError => Err(anyhow!("runtime error")),
            InterpretResult::CompileError => unreachable!(),
        }
    };

    let output_bytes = output_buffer.borrow().clone();
    let output_string = String::from_utf8(output_bytes).unwrap_or_default();
    result.map(|_| output_string)
}
