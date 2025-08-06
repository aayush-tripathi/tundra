// src/lib.rs
pub mod bytecode;
pub mod compiler;
pub mod jit;
pub mod lexer;
pub mod vm;

pub use crate::vm::interpretresult::InterpretResult;
use crate::{
    bytecode::chunk::Chunk, compiler::compiler::Compiler, lexer::scanner::Scanner, vm::vm::VM,
};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

/// A custom Write implementation that captures output to a shared buffer
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

pub fn run(src: &str) -> Result<String> {
    //  a shared buffer for capturing output
    let output_buffer = Rc::new(RefCell::new(Vec::<u8>::new()));

    let output_capture = Box::new(OutputCapture::new(output_buffer.clone()));
    let output_ref: &'static mut dyn std::io::Write = Box::leak(output_capture);

    // ---------- compile ----------
    let _tokens = Scanner::new(src.to_string()).scan_tokens();
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    if !compiler.compile(src) {
        return Err(anyhow!("compile error"));
    }

    // ---------- execute ----------
    let result = {
        let mut vm = VM::new_interpreter_only(chunk, output_ref);
        match vm.run() {
            InterpretResult::Ok => Ok(()),
            InterpretResult::RuntimeError => Err(anyhow!("runtime error")),
            InterpretResult::CompileError => unreachable!(),
        }
    };
    let output_bytes = {
        let buffer = output_buffer.borrow();
        buffer.clone()
    };
    let output_string = String::from_utf8(output_bytes).unwrap_or_default();
    result.map(|_| output_string)
}
