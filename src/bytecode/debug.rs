use crate::bytecode::opcode::OpCode;
use super::{chunk::Chunk, value::Value};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    
    let instruction = &chunk.code[offset];
    let line = chunk.get_line(offset).unwrap_or(-1);
    print!("Line {:04} ", line);

    match instruction {
        OpCode::Return(reg) => simple_instruction("RETURN", *reg, offset),
        OpCode::LoadConstant(reg, value) => constant_instruction("LOAD_CONSTANT", *reg, value, offset),
        OpCode::DefineGlobal(reg, name) => global_instruction("DEFINE_GLOBAL", *reg, name, offset),
        OpCode::GetGlobal(reg, name) => global_instruction("GET_GLOBAL", *reg, name, offset),
        OpCode::SetGlobal(reg, name) => global_instruction("SET_GLOBAL", *reg, name, offset),
        OpCode::SetLocal(dest, src) => local_instruction("SET_LOCAL", *dest, *src, offset),
        OpCode::GetLocal(dest, src) => local_instruction("GET_LOCAL", *dest, *src, offset),
        OpCode::JumpIfFalse(reg, index) => jump_instruction("JUMP_IF_FALSE", *reg, *index, offset),
        OpCode::Jump(index) => jump_instruction("JUMP", 0, *index, offset), // No reg needed
        OpCode::Loop(index) => jump_instruction("LOOP", 0, *index, offset), // No reg needed
        OpCode::Move(dest, src) => local_instruction("MOVE", *dest, *src, offset),
        OpCode::True(reg) => simple_instruction("TRUE", *reg, offset),
        OpCode::False(reg) => simple_instruction("FALSE", *reg, offset),
        OpCode::Pop(reg) => simple_instruction("POP", *reg, offset),
        OpCode::None(reg) => simple_instruction("NONE", *reg, offset),
        OpCode::Add(dest, src1, src2) => triple_instruction("ADD", *dest, *src1, *src2, offset),
        OpCode::Subtract(dest, src1, src2) => triple_instruction("SUBTRACT", *dest, *src1, *src2, offset),
        OpCode::Multiply(dest, src1, src2) => triple_instruction("MULTIPLY", *dest, *src1, *src2, offset),
        OpCode::Divide(dest, src1, src2) => triple_instruction("DIVIDE", *dest, *src1, *src2, offset),
        OpCode::Exponentiate(dest, src1, src2)=> triple_instruction("EXPONENT",*dest, *src1, *src2, offset),
        OpCode::IntDivide(dest, src1, src2)=> triple_instruction("INTDIV",*dest, *src1, *src2, offset),
        OpCode::BitwiseNot(dest, src) => local_instruction("NOT", *dest, *src, offset),
        OpCode::Negate(dest, src) => local_instruction("NEGATE", *dest, *src, offset),
        OpCode::Print(reg) => simple_instruction("PRINT", *reg, offset),
        OpCode::Equal(dest, src1, src2) => triple_instruction("EQUAL", *dest, *src1, *src2, offset),
        OpCode::Greater(dest, src1, src2) => triple_instruction("GREATER", *dest, *src1, *src2, offset),
        OpCode::Less(dest, src1, src2) => triple_instruction("LESS", *dest, *src1, *src2, offset),
        OpCode::BitwiseAnd(dest, src1, src2) => triple_instruction("AND", *dest, *src1, *src2,offset),
        OpCode::BitwiseOr(dest, src1, src2) => triple_instruction("OR", *dest, *src1, *src2,offset),
        OpCode::BitwiseXor(dest, src1, src2) => triple_instruction("XOR", *dest, *src1, *src2,offset),
        OpCode::GreaterEqual(dest,src1 ,src2 )=> triple_instruction("GREATER_EQ", *dest, *src1, *src2, offset),
        OpCode::LessEqual(dest,src1 ,src2 )=> triple_instruction("LESSER_EQ", *dest, *src1, *src2, offset),
        OpCode::NotEqual(dest,src1 ,src2 )=> triple_instruction("NOT_EQUAL", *dest, *src1, *src2, offset),
    }
}

fn simple_instruction(name: &str, reg: usize, offset: usize) -> usize {
    println!("{} r{}", name, reg);
    offset + 1
}

fn constant_instruction(name: &str, reg: usize, value: &Value, offset: usize) -> usize {
    println!("{} r{} {:?}", name, reg, value);
    offset + 1
}

fn global_instruction(name: &str, reg: usize, variable: &str, offset: usize) -> usize {
    println!("{} r{} {}", name, reg, variable);
    offset + 1
}

fn local_instruction(name: &str, dest: usize, src: usize, offset: usize) -> usize {
    println!("{} r{} r{}", name, dest, src);
    offset + 1
}

fn jump_instruction(name: &str, reg: usize, index: usize, offset: usize) -> usize {
    if reg == 0 {
        println!("{} {}", name, index);
    } else {
        println!("{} r{} {}", name, reg, index);
    }
    offset + 1
}

fn triple_instruction(name: &str, dest: usize, src1: usize, src2: usize, offset: usize) -> usize {
    println!("{} r{} r{} r{}", name, dest, src1, src2);
    offset + 1
}
