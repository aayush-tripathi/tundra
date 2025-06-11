// bytecode/opcode.rs

use super::value::Value;
use std::fmt;
///  The Valid Operations `Tundra` can perform
#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    LoadConstant(usize, Value),
    DefineGlobal(usize, String),
    GetGlobal(usize, String),
    SetGlobal(usize, String),
    GetLocal(usize, usize),
    SetLocal(usize, usize),
    JumpIfFalse(usize, usize),
    Jump(usize),
    Loop(usize),
    Move(usize, usize),
    True(usize),
    False(usize),
    Pop(usize),
    None(usize),
    Add(usize, usize, usize),
    Subtract(usize, usize, usize),
    Multiply(usize, usize, usize),
    Divide(usize, usize, usize),
    Exponentiate(usize, usize, usize),
    IntDivide(usize, usize, usize),
    Mod(usize, usize, usize),
    Return(usize),
    BitwiseNot(usize, usize),
    Negate(usize, usize),
    Print(usize),
    Equal(usize, usize, usize),
    Greater(usize, usize, usize),
    Less(usize, usize, usize),
    BitwiseAnd(usize, usize, usize),
    BitwiseOr(usize, usize, usize),
    BitwiseXor(usize, usize, usize),
    GreaterEqual(usize, usize, usize),
    LessEqual(usize, usize, usize),
    NotEqual(usize, usize, usize),
    Call(usize, usize, usize),
    NewArray(usize, usize),             // dest, length-reg
    GetIndex(usize, usize, usize),      // dest, array-reg, index-reg
    SetIndex(usize, usize, usize),      // array-reg, index-reg, value-reg
    IncLoopIfLess(usize, usize, usize), // idx_reg, limit_reg, target_pc
}

impl OpCode {
    /// A compact numeric tag for each variant (useful for a real byte-buffer backend)
    pub fn tag(&self) -> u8 {
        match self {
            OpCode::LoadConstant(_, _) => 0x01,
            OpCode::DefineGlobal(_, _) => 0x02,
            OpCode::GetGlobal(_, _) => 0x03,
            OpCode::SetGlobal(_, _) => 0x04,
            OpCode::GetLocal(_, _) => 0x05,
            OpCode::SetLocal(_, _) => 0x06,
            OpCode::JumpIfFalse(_, _) => 0x07,
            OpCode::Jump(_) => 0x08,
            OpCode::Loop(_) => 0x09,
            OpCode::Move(_, _) => 0x0A,
            OpCode::True(_) => 0x0B,
            OpCode::False(_) => 0x0C,
            OpCode::Pop(_) => 0x0D,
            OpCode::None(_) => 0x0E,
            OpCode::Add(_, _, _) => 0x0F,
            OpCode::Subtract(_, _, _) => 0x10,
            OpCode::Multiply(_, _, _) => 0x11,
            OpCode::Divide(_, _, _) => 0x12,
            OpCode::Exponentiate(_, _, _) => 0x13,
            OpCode::IntDivide(_, _, _) => 0x14,
            OpCode::Return(_) => 0x15,
            OpCode::BitwiseNot(_, _) => 0x16,
            OpCode::Negate(_, _) => 0x17,
            OpCode::Print(_) => 0x18,
            OpCode::Equal(_, _, _) => 0x19,
            OpCode::Greater(_, _, _) => 0x1A,
            OpCode::Less(_, _, _) => 0x1B,
            OpCode::BitwiseAnd(_, _, _) => 0x1C,
            OpCode::BitwiseOr(_, _, _) => 0x1D,
            OpCode::BitwiseXor(_, _, _) => 0x1E,
            OpCode::GreaterEqual(_, _, _) => 0x1F,
            OpCode::LessEqual(_, _, _) => 0x20,
            OpCode::NotEqual(_, _, _) => 0x21,
            OpCode::Call(_, _, _) => 0x22,
            OpCode::Mod(_, _, _) => 0x23,
            OpCode::NewArray(_, _) => 0x24,
            OpCode::GetIndex(_, _, _) => 0x25,
            OpCode::SetIndex(_, _, _) => 0x26,
            OpCode::IncLoopIfLess(_, _, _) => 0x27,
        }
    }
    ///Convert to bytes (For Debugging)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![self.tag()];
        match self {
            OpCode::LoadConstant(dest, val) => {
                buf.extend(&(*dest as u32).to_le_bytes());
                buf.extend(&val.to_bytes());
            }
            OpCode::DefineGlobal(reg, name)
            | OpCode::GetGlobal(reg, name)
            | OpCode::SetGlobal(reg, name) => {
                buf.extend(&(*reg as u32).to_le_bytes());
                let bytes = name.as_bytes();
                buf.extend(&(bytes.len() as u32).to_le_bytes());
                buf.extend(bytes);
            }
            OpCode::GetLocal(d, s)
            | OpCode::SetLocal(d, s)
            | OpCode::Move(d, s)
            | OpCode::BitwiseNot(d, s)
            | OpCode::Negate(d, s) => {
                buf.extend(&(*d as u32).to_le_bytes());
                buf.extend(&(*s as u32).to_le_bytes());
            }
            OpCode::JumpIfFalse(r, idx) => {
                buf.extend(&(*r as u32).to_le_bytes());
                buf.extend(&(*idx as u32).to_le_bytes());
            }
            OpCode::Jump(idx) | OpCode::Loop(idx) => {
                buf.extend(&(*idx as u32).to_le_bytes());
            }
            OpCode::True(r)
            | OpCode::False(r)
            | OpCode::Pop(r)
            | OpCode::None(r)
            | OpCode::Return(r)
            | OpCode::Print(r) => {
                buf.extend(&(*r as u32).to_le_bytes());
            }
            OpCode::Add(d, a, b)
            | OpCode::Subtract(d, a, b)
            | OpCode::Multiply(d, a, b)
            | OpCode::Divide(d, a, b)
            | OpCode::Exponentiate(d, a, b)
            | OpCode::IntDivide(d, a, b)
            | OpCode::Mod(d, a, b)
            | OpCode::Equal(d, a, b)
            | OpCode::Greater(d, a, b)
            | OpCode::Less(d, a, b)
            | OpCode::BitwiseAnd(d, a, b)
            | OpCode::BitwiseOr(d, a, b)
            | OpCode::BitwiseXor(d, a, b)
            | OpCode::GreaterEqual(d, a, b)
            | OpCode::LessEqual(d, a, b)
            | OpCode::NotEqual(d, a, b) => {
                buf.extend(&(*d as u32).to_le_bytes());
                buf.extend(&(*a as u32).to_le_bytes());
                buf.extend(&(*b as u32).to_le_bytes());
            }
            OpCode::Call(d, c, a) => {
                buf.extend(&(*d as u32).to_le_bytes());
                buf.extend(&(*c as u32).to_le_bytes());
                buf.extend(&(*a as u32).to_le_bytes());
            }
            OpCode::NewArray(dest, len) => {
                buf.extend(&(*dest as u32).to_le_bytes());
                buf.extend(&(*len as u32).to_le_bytes());
            }
            OpCode::GetIndex(d, arr, idx) | OpCode::SetIndex(d, arr, idx) => {
                buf.extend(&(*d as u32).to_le_bytes());
                buf.extend(&(*arr as u32).to_le_bytes());
                buf.extend(&(*idx as u32).to_le_bytes());
            }
            OpCode::IncLoopIfLess(idx, limit, target) => {
                buf.extend(&(*idx as u32).to_le_bytes());
                buf.extend(&(*limit as u32).to_le_bytes());
                buf.extend(&(*target as u32).to_le_bytes());
            }
            _ => {}
        }
        buf
    }
    /// Return every register index this opcode reads or writes.
    pub fn regs(&self) -> Vec<usize> {
        use OpCode::*;
        match self {
            LoadConstant(d, _)
            | DefineGlobal(d, _)
            | GetGlobal(d, _)
            | True(d)
            | False(d)
            | Pop(d)
            | None(d) => vec![*d],

            GetLocal(d, _) | SetLocal(d, _) => vec![*d],

            SetGlobal(r, _) => vec![*r],

            Add(d, a, b)
            | Subtract(d, a, b)
            | Multiply(d, a, b)
            | Divide(d, a, b)
            | IntDivide(d, a, b)
            | Exponentiate(d, a, b)
            | Mod(d, a, b)
            | BitwiseAnd(d, a, b)
            | BitwiseOr(d, a, b)
            | BitwiseXor(d, a, b)
            | Equal(d, a, b)
            | NotEqual(d, a, b)
            | Greater(d, a, b)
            | GreaterEqual(d, a, b)
            | Less(d, a, b)
            | LessEqual(d, a, b) => vec![*d, *a, *b],

            Move(d, s) => vec![*d, *s],
            Call(d, c, _) => vec![*d, *c],
            NewArray(d, l) => vec![*d, *l],
            GetIndex(d, a, i) => vec![*d, *a, *i],
            SetIndex(a, i, v) => vec![*a, *i, *v],

            JumpIfFalse(r, _) => vec![*r],
            Loop(_) | Jump(_) => vec![],
            _ => vec![],
        }
    }
}
