#[derive(Debug, Clone)]
pub enum TokenType {
 
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Slash, Star,
    Percent, Caret, Ampersand, Pipe,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    SlashSlash, StarStar,
    MinusMinus, PlusPlus,
    
  
    Identifier, String,
    Int, LongInt, Float, Bool, Char,


    And, Class, Else, False,
    For, Fun, If, None, Or,
    Print, Return, Super, This,
    True, Var, While,
    Not,

    Error, 
    EOF,

    _Default,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub start_line: i32,
    pub end_line: i32,
}
