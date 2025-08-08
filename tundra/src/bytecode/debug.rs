// bytecode/debug.rs

use crate::bytecode::chunk::Chunk;
use std::fmt::Write as _;
/// A one‐pass disassembler
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instr) in chunk.code.iter().enumerate() {
        let line = chunk.get_line(offset).unwrap_or(-1);
        println!("{:04} Line {:4}    {:?}", offset, line, instr);
    }
}
pub fn disassemble_to_string(chunk: &Chunk, name: &str) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "== {} ==", name);
    for (offset, instr) in chunk.code.iter().enumerate() {
        let line = chunk.get_line(offset).unwrap_or(-1);
        let _ = writeln!(s, "{:04} Line {:4}    {:?}", offset, line, instr);
    }
    s
}
