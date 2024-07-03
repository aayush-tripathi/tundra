use std::fmt;

use super::value::Value;

#[derive(Clone,Debug)]

pub enum OpCode {
    LoadConstant(usize, Value),    // Load a constant value into a register
    DefineGlobal(usize, String),   // Define a global variable from a register value
    GetGlobal(usize, String),      // Load a global variable into a register
    SetGlobal(usize, String),      // Set a global variable from a register value
    GetLocal(usize, usize),        // Load a local variable into a register
    SetLocal(usize, usize),        // Set a local variable from a register value
    JumpIfFalse(usize, usize),     // Jump if the value in a register is false
    Jump(usize),                   // Unconditional jump
    Loop(usize),                   // Loop to a specific instruction
    Move(usize, usize),            // Move value from one register to another
    True(usize),                   // Load true into a register
    False(usize),                  // Load false into a register
    Pop(usize),                    // Clear a register
    None(usize),                   // Load None into a register
    Add(usize, usize, usize),      // Add values from two registers and store in another register
    Subtract(usize, usize, usize), // Subtract values from two registers and store in another register
    Multiply(usize, usize, usize), // Multiply values from two registers and store in another register
    Divide(usize, usize, usize),   // Divide values from two registers and store in another register
    Exponentiate(usize, usize, usize),//Raise the value in a register to the power of another register and store the result in another register
    IntDivide(usize,usize,usize),// Divide values and convert the answer to an int. Equivalent of Floor
    Return(usize),                 // Return the value in a register
    BitwiseNot(usize, usize),             // Logical NOT on a register and store in another register
    Negate(usize, usize),          // Negate a value in a register and store in another register
    Print(usize),                  // Print the value in a register
    Equal(usize, usize, usize),    // Compare values from two registers for equality and store in another register
    Greater(usize, usize, usize),  // Compare if value in one register is greater than another and store in a register
    GreaterEqual(usize, usize, usize), // Compare if value in one registers is greater than or equal to another and store
    Less(usize, usize, usize),     // Compare if value in one register is less than another and store in a register
    LessEqual(usize, usize, usize), // Compare if value in one register is less than or equal to another and store
    NotEqual(usize, usize, usize), // Compare if value in one register is not equal to another and store in a register
    BitwiseAnd(usize, usize, usize), // Added
    BitwiseOr(usize, usize, usize),  // Added
    BitwiseXor(usize, usize, usize), // Added
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::LoadConstant(reg, val) => write!(f, "LoadConstant r{} {}", reg, val),
            OpCode::DefineGlobal(reg, name) => write!(f, "DefineGlobal r{} {}", reg, name),
            OpCode::GetGlobal(reg, name) => write!(f, "GetGlobal r{} {}", reg, name),
            OpCode::SetGlobal(reg, name) => write!(f, "SetGlobal r{} {}", reg, name),
            OpCode::GetLocal(dest, src) => write!(f, "GetLocal r{} r{}", dest, src),
            OpCode::SetLocal(dest, src) => write!(f, "SetLocal r{} r{}", dest, src),
            OpCode::JumpIfFalse(reg, offset) => write!(f, "JumpIfFalse r{} {}", reg, offset),
            OpCode::Jump(offset) => write!(f, "Jump {}", offset),
            OpCode::Loop(offset) => write!(f, "Loop {}", offset),
            OpCode::Move(dest, src) => write!(f, "Move r{} r{}", dest, src),
            OpCode::True(reg) => write!(f, "True r{}", reg),
            OpCode::False(reg) => write!(f, "False r{}", reg),
            OpCode::Pop(reg) => write!(f, "Pop r{}", reg),
            OpCode::None(reg) => write!(f, "None r{}", reg),
            OpCode::Add(dest, src1, src2) => write!(f, "Add r{} r{} r{}", dest, src1, src2),
            OpCode::Subtract(dest, src1, src2) => write!(f, "Subtract r{} r{} r{}", dest, src1, src2),
            OpCode::Multiply(dest, src1, src2) => write!(f, "Multiply r{} r{} r{}", dest, src1, src2),
            OpCode::Divide(dest, src1, src2) => write!(f, "Divide r{} r{} r{}", dest, src1, src2),
            OpCode::Exponentiate(dest, src1, src2) => write!(f, "Exponentiate r{} r{} r{}", dest, src1, src2),
            OpCode::IntDivide(dest, src1, src2) => write!(f, "IntDivide r{} r{} r{}", dest, src1, src2),
            OpCode::Return(reg) => write!(f, "Return r{}", reg),
            OpCode::BitwiseNot(dest, src) => write!(f, "Not r{} r{}", dest, src),
            OpCode::Negate(dest, src) => write!(f, "Negate r{} r{}", dest, src),
            OpCode::Print(reg) => write!(f, "Print r{}", reg),
            OpCode::Equal(dest, src1, src2) => write!(f, "Equal r{} r{} r{}", dest, src1, src2),
            OpCode::Greater(dest, src1, src2) => write!(f, "Greater r{} r{} r{}", dest, src1, src2),
            OpCode::Less(dest, src1, src2) => write!(f, "Less r{} r{} r{}", dest, src1, src2),
            OpCode::BitwiseAnd(dest, src1, src2)=>write!(f, "Bit And r{} r{} r{}", dest, src1, src2),
            OpCode::BitwiseOr(dest, src1, src2)=>write!(f, "Bit Or r{} r{} r{}", dest, src1, src2),
            OpCode::BitwiseXor(dest, src1, src2)=>write!(f, "Bit XOR r{} r{} r{}", dest, src1, src2),
            OpCode::GreaterEqual(dest, src1, src2) => write!(f, "GreaterEqual r{} r{} r{}", dest, src1, src2),
            OpCode::LessEqual(dest, src1, src2) => write!(f, "LessEqual r{} r{} r{}", dest, src1, src2),
            OpCode::NotEqual(dest, src1, src2) => write!(f, "NotEqual r{} r{} r{}", dest, src1, src2),
        }
    }
}


#[derive(Clone)]
pub struct CodeLine {
    pub code: OpCode,
}

impl fmt::Display for CodeLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ Code: {} }}", self.code)
    }
}
