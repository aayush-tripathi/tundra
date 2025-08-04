// bytecode/chunk.rs

use super::opcode::OpCode;
use crate::bytecode::value::Value;

/// A compiled “chunk” of code:  
/// - `code` holds high-level `OpCode` instructions  
/// - `constants` is the literal pool  
/// - `lines[i]` is the source‐line for `code[i]`
/// - `max_register` to allocate CraneLift Variables
#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
    pub max_register: usize,
}

impl Chunk {
    /// Constructor
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            max_register: 0,
        }
    }

    /// Emit one instruction at `line`
    pub fn write(&mut self, opcode: OpCode, line: i32) {
        self.code.push(opcode);
        self.lines.push(line);
    }

    /// Add a constant and return its index
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// O(1) lookup of source line for instruction `index`
    pub fn get_line(&self, index: usize) -> Option<i32> {
        self.lines.get(index).cloned()
    }
}
