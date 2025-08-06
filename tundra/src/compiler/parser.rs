// src/compiler/parser.rs

use super::compiler::Compiler;
use crate::lexer::token::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current_idx: usize,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        let dummy = Token::new(TokenType::EOF, String::new(), 0, 0);
        Parser {
            tokens: Vec::new(),
            current_idx: 0,
            current: dummy.clone(),
            previous: dummy,
            had_error: false,
            panic_mode: false,
        }
    }

    /// Advance one token from the pre-scanned list.
    pub fn advance_token(&mut self) {
        self.previous = self.current.clone();
        if self.current_idx < self.tokens.len() {
            self.current = self.tokens[self.current_idx].clone();
        } else {
            self.current = Token::new(TokenType::EOF, String::new(), 0, 0);
        }
        self.current_idx += 1;
    }
}

/// How tightly different constructs bind.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    BitwiseXor,
    BitwiseOr,
    BitwiseAnd,
    Term,
    Factor,
    Unary,
    Exponent,
    Call,
    Primary,
}

impl Precedence {
    pub fn next_rule(rule: Precedence) -> Precedence {
        use Precedence::*;
        match rule {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => BitwiseXor,
            BitwiseXor => BitwiseOr,
            BitwiseOr => BitwiseAnd,
            BitwiseAnd => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Exponent,
            Exponent => Call,
            Call => Primary,
            Primary => panic!("No precedence above Primary"),
        }
    }
}

/// A local variable.
#[derive(Clone)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub initialized: bool,
    pub register: usize,
}

/// The signature of any Pratt-style prefix or infix parse function.
pub type ParserFunction = fn(&mut Compiler, can_assign: bool);

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub prefix: Option<ParserFunction>,
    pub infix: Option<ParserFunction>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub const fn prefix(prefix: ParserFunction, precedence: Precedence) -> Self {
        ParseRule {
            prefix: Some(prefix),
            infix: None,
            precedence,
        }
    }
    pub const fn infix(infix: ParserFunction, precedence: Precedence) -> Self {
        ParseRule {
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }
    pub const fn both(
        prefix: ParserFunction,
        infix: ParserFunction,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            prefix: Some(prefix),
            infix: Some(infix),
            precedence,
        }
    }
    pub const fn neither() -> Self {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }
}

pub fn get_rule(token: TokenType) -> ParseRule {
    use Precedence::*;
    use TokenType::*;
    match token {
        // grouping, function calls
        LeftParen => ParseRule::both(Compiler::grouping, Compiler::call, Call),
        // array‐literal in prefix, indexing in infix:
        LeftBracket => ParseRule::both(Compiler::array_literal, Compiler::index, Call),

        // a whole bunch of infixes:
        Plus => ParseRule::infix(Compiler::binary, Term),
        Minus => ParseRule::both(Compiler::unary, Compiler::binary, Term),
        Star => ParseRule::infix(Compiler::binary, Factor),
        Slash => ParseRule::infix(Compiler::binary, Factor),
        SlashSlash => ParseRule::infix(Compiler::binary, Factor),
        Percent => ParseRule::infix(Compiler::binary, Factor),
        StarStar => ParseRule::infix(Compiler::binary, Exponent),

        Caret => ParseRule::infix(Compiler::binary, BitwiseXor),
        Ampersand => ParseRule::infix(Compiler::binary, BitwiseAnd),
        Pipe => ParseRule::infix(Compiler::binary, BitwiseOr),

        BangEqual => ParseRule::infix(Compiler::binary, Equality),
        EqualEqual => ParseRule::infix(Compiler::binary, Equality),
        Greater => ParseRule::infix(Compiler::binary, Comparison),
        GreaterEqual => ParseRule::infix(Compiler::binary, Comparison),
        Less => ParseRule::infix(Compiler::binary, Comparison),
        LessEqual => ParseRule::infix(Compiler::binary, Comparison),

        PlusPlus => ParseRule::prefix(Compiler::prefix_incdec, Unary),
        MinusMinus => ParseRule::prefix(Compiler::prefix_incdec, Unary),

        // literals
        Int => ParseRule::prefix(Compiler::literal_int, Primary),
        Float => ParseRule::prefix(Compiler::literal_float, Primary),
        String => ParseRule::prefix(Compiler::literal_string, Primary),
        Char => ParseRule::prefix(Compiler::literal_char, Primary),
        True | False => ParseRule::prefix(Compiler::literal_bool, Primary),
        TokenType::None => ParseRule::prefix(Compiler::literal_none, Primary),
        Identifier => ParseRule::prefix(Compiler::variable, Primary),

        Not => ParseRule::prefix(Compiler::unary, Unary),
        TokenType::And => ParseRule::infix(Compiler::binary, Precedence::And),
        TokenType::Or => ParseRule::infix(Compiler::binary, Precedence::Or),

        _ => ParseRule::neither(),
    }
}
