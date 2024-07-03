use crate::{bytecode::{chunk::Chunk, opcode::OpCode, value::Value}, lexer::{scanner::Scanner, token::{Token, TokenType}}};

use super::{parser::{Local, Parser}, register::RegisterAllocator};




pub struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: &'a mut Chunk,
    scope_depth: usize,
    locals: Vec<Local>,
    register_allocator: RegisterAllocator,
}

impl Compiler<'_> {
    pub fn new(chunk: &mut Chunk) -> Compiler {
        Compiler {
            scanner: Scanner::new("".to_string()),
            parser: Parser::new(),
            compiling_chunk: chunk,
            locals: Vec::new(),
            scope_depth: 0,
            register_allocator: RegisterAllocator::new(), 
        }
    }
}