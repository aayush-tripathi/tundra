// bytecode/debug.rs

use crate::bytecode::chunk::Chunk;

/// A one‐pass disassembler: we print each instruction via its Debug impl.
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (offset, instr) in chunk.code.iter().enumerate() {
        let line = chunk.get_line(offset).unwrap_or(-1);
        // Use {:?} since OpCode implements Debug, not Display
        println!("{:04} Line {:4}    {:?}", offset, line, instr);
    }
}
