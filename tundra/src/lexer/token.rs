
// src/lexer/token.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen,
    LeftBracket, RightBracket,
    Comma, Dot, Colon,

    // Operators
    Plus, Minus, Star, Slash, Percent, Caret,
    Ampersand, Pipe,
    PlusPlus, MinusMinus, StarStar, SlashSlash,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=
    SlashSlashEqual, // //=
    PercentEqual, // %=
    StarStarEqual,// **=
    AmpersandEqual, // &=
    PipeEqual,      // |=
    CaretEqual,     // ^=

    // Literals
    Identifier, String, Char,
    Int, Float,

    // Keywords
    And, Class, Else, False,
    For, Fun, If, None, Or,
    Print, Return, Super, This,
    True, Var, While, Not,In,

    // Pythonic layout
    Newline, Indent, Dedent,
    Break,Continue,

    Error,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub start_line: i32,
    pub end_line: i32,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, start_line: i32, end_line: i32) -> Self {
        Token { token, lexeme, start_line, end_line }
    }
}
