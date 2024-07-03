use crate::lexer::token::{Token, TokenType};

use super::compiler::Compiler;

pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { 
            current: Token { 
                token: TokenType::_Default,
                end_line: 0,
                lexeme: "".to_string(),
                start_line: 0,
            },
            previous: Token {
                token: TokenType:: _Default,
                end_line: 0,
                lexeme: "".to_string(),
                start_line: 0,
            },
            had_error: false,
            panic_mode: false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    BitwiseOr,   // |
    BitwiseAnd,  // &
    Term,        // + -
    Factor,      // * /
    Unary,       // ! - ++ --
    Exponent,    // **
    Call,        // . ()
    Primary,
}

impl Precedence {
    pub fn next_rule(parse_rule: ParseRule) -> Precedence {
        match parse_rule.precedence {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Exponent,
            Precedence::Exponent => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => panic!("No matching rule higher than Primary"),
        }
    }
}
#[derive(Clone)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub initialized: bool,
}
pub type ParserFunction = fn(&mut Compiler, bool) -> ();

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub infix: Option<ParserFunction>,
    pub prefix: Option<ParserFunction>,
    pub precedence: Precedence,
}

impl ParseRule {
    const fn infix(infix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: Some(infix),
            precedence,
            prefix: None,
        }
    }
    
    const fn prefix(prefix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: None,
            prefix: Some(prefix),
            precedence,
        }
    }

    const fn both(prefix: ParserFunction, infix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: Some(infix),
            prefix: Some(prefix),
            precedence,
        }
    }

    const fn neither() -> ParseRule {
        ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        }
    }
}

// maps are heap allocated so this is faster
/*pub const RULES: [ParseRule; 50] = [
    ParseRule::prefix(|compiler, can_assign| compiler.grouping(can_assign), Precedence::None), // LeftParen
    ParseRule::neither(), // RightParen
    ParseRule::neither(), // LeftBrace
    ParseRule::neither(), // RightBrace
    ParseRule::neither(), // Comma
    ParseRule::neither(), // Dot
    ParseRule::both(|compiler, can_assign| compiler.unary(can_assign), |compiler, can_assign| compiler.binary(can_assign), Precedence::Term), // Minus
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Term), // Plus
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), // Slash
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), // Star
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), // Percent
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Exponent), // Caret
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::BitwiseAnd), // Ampersand
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::BitwiseOr), // Pipe
    ParseRule::prefix(|compiler, can_assign| compiler.unary(can_assign), Precedence::Unary), // Bang
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Equality), // BangEqual
    ParseRule::neither(), // Equal
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Equality), // EqualEqual
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), // Greater
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), // GreaterEqual
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), // Less
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), // LessEqual
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Exponent), // StarStar
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), // SlashSlash
    ParseRule::prefix(|compiler, can_assign| compiler.unary(can_assign), Precedence::Unary), // MinusMinus
    ParseRule::prefix(|compiler, can_assign| compiler.unary(can_assign), Precedence::Unary), // PlusPlus
    ParseRule::prefix(|compiler, can_assign| compiler.variable(can_assign), Precedence::None), // Identifier
    ParseRule::prefix(|compiler, can_assign| compiler.string(can_assign), Precedence::None), // String
    ParseRule::prefix(|compiler, can_assign| compiler.int(can_assign), Precedence::None), // Int
    ParseRule::prefix(|compiler, can_assign| compiler.float(can_assign), Precedence::None), // Float
    ParseRule::prefix(|compiler, can_assign| compiler.bool(can_assign), Precedence::None), // Bool
    ParseRule::prefix(|compiler, can_assign| compiler.char(can_assign), Precedence::None), // Char
    ParseRule::infix(|compiler, _can_assign| compiler.and(_can_assign), Precedence::And), // And
    ParseRule::neither(), // Class
    ParseRule::neither(), // Else
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // False
    ParseRule::neither(), // For
    ParseRule::neither(), // Fun
    ParseRule::neither(), // If
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // None
    ParseRule::infix(|compiler, _can_assign| compiler.or(_can_assign), Precedence::Or), // Or
    ParseRule::neither(), // Print
    ParseRule::neither(), // Return
    ParseRule::neither(), // Super
    ParseRule::neither(), // This
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // True
    ParseRule::neither(), // Var
    ParseRule::neither(), // While
    ParseRule::neither(), // Error
    ParseRule::neither(), // EOF
];*/