use crate::bytecode::value::Value;

use super::opcode::OpCode;


pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<(i32, usize)>, //RUN-LENGTH DECODING?????
}

impl Chunk {
    pub fn new_chunk() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, code: OpCode, line: i32) {
        self.code.push(code);
        if let Some((last_line, count)) = self.lines.last_mut() {
            if *last_line == line {
                *count += 1;
            } else {
                self.lines.push((line, 1));
            }
        } else {
            self.lines.push((line, 1));
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_line(&self, index: usize) -> Option<i32> {
        let mut current_index = 0;
        for &(line, count) in &self.lines {
            if index < current_index + count {
                return Some(line);
            }
            current_index += count;
        }
        None
    }
}
