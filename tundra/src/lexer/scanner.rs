// lexer/scanner.rs

use super::helper::TrieNode;
use super::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    start_line: i32,
    current_line: i32,
    keyword_trie: TrieNode,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            start_line: 1,
            current_line: 1,
            keyword_trie: Self::initialize_keyword_trie(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut indents = vec![0];
        let mut at_line_start = true;

        while !self.is_at_end() {
            if at_line_start && self.peek() == '\n' {
                self.advance();
                let tok = Token::new(TokenType::Newline, "\n".into(), self.current_line, {
                    self.current_line += 1;
                    self.current_line - 1
                });
                tokens.push(tok);

                at_line_start = true;
                continue;
            }
            if at_line_start {
                let mut count = 0;
                let mut saw_space = false;
                let mut saw_tab = false;
                while self.peek() == ' ' || self.peek() == '\t' {
                    if self.peek() == ' ' {
                        saw_space = true;
                    } else {
                        saw_tab = true;
                    }
                    self.advance();
                    count += 1;
                }
                if saw_space && saw_tab {
                    tokens.push(Token::new(
                        TokenType::Error,
                        "Mixed tabs and spaces in indentation".into(),
                        self.current_line,
                        self.current_line,
                    ));
                } else {
                    let last = *indents.last().unwrap();
                    if count > last {
                        indents.push(count);
                        tokens.push(Token::new(
                            TokenType::Indent,
                            String::new(),
                            self.current_line,
                            self.current_line,
                        ));
                    } else {
                        while count < *indents.last().unwrap() {
                            indents.pop();
                            tokens.push(Token::new(
                                TokenType::Dedent,
                                String::new(),
                                self.current_line,
                                self.current_line,
                            ));
                        }
                    }
                }
            }

            if let Some(tok) = self.scan_token() {
                at_line_start = tok.token == TokenType::Newline;
                tokens.push(tok);
            }
        }
        if !matches!(tokens.last().map(|t| t.token), Some(TokenType::Newline)) {
            tokens.push(Token::new(
                TokenType::Newline,
                "\n".into(),
                self.current_line,
                self.current_line,
            ));
        }
        while indents.len() > 1 {
            indents.pop();
            tokens.push(Token::new(
                TokenType::Dedent,
                String::new(),
                self.current_line,
                self.current_line,
            ));
        }
        tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            self.current_line,
            self.current_line,
        ));
        tokens
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        loop {
            if self.is_at_end() {
                return None;
            }
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '#' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                '\n' => {
                    self.advance();
                    let tok = Token::new(TokenType::Newline, "\n".into(), self.current_line, {
                        self.current_line += 1;
                        self.current_line - 1
                    });
                    return Some(tok);
                }
                _ => break,
            }
        }

        self.start = self.current;
        self.start_line = self.current_line;

        if self.is_at_end() {
            return None;
        }

        let c = self.advance();
        let tok = match c {
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,

            '+' => {
                if self.match_char('+') {
                    TokenType::PlusPlus
                } else if self.match_char('=') {
                    TokenType::PlusEqual
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                if self.match_char('-') {
                    TokenType::MinusMinus
                } else if self.match_char('=') {
                    TokenType::MinusEqual
                } else {
                    TokenType::Minus
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        TokenType::StarStarEqual
                    } else {
                        TokenType::StarStar
                    }
                } else if self.match_char('=') {
                    TokenType::StarEqual
                } else {
                    TokenType::Star
                }
            }
            '/' => {
                if self.match_char('/') {
                    if self.match_char('=') {
                        TokenType::SlashSlashEqual
                    } else {
                        TokenType::SlashSlash
                    }
                } else if self.match_char('=') {
                    TokenType::SlashEqual
                } else {
                    TokenType::Slash
                }
            }
            '%' => {
                if self.match_char('=') {
                    TokenType::PercentEqual
                } else {
                    TokenType::Percent
                }
            }

            '^' => {
                if self.match_char('=') {
                    TokenType::CaretEqual
                } else {
                    TokenType::Caret
                }
            }
            '&' => {
                if self.match_char('=') {
                    TokenType::AmpersandEqual
                } else {
                    TokenType::Ampersand
                }
            }
            '|' => {
                if self.match_char('=') {
                    TokenType::PipeEqual
                } else {
                    TokenType::Pipe
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '"' => return Some(self.string()),
            '\'' => return Some(self.char_literal()),
            c if c.is_ascii_digit() => return Some(self.number()),
            c if Self::is_alpha(c) => return Some(self.identifier()),
            _ => TokenType::Error,
        };

        Some(self.add_token(tok))
    }

    fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.current_line += 1;
                    }
                    self.advance();
                }
                continue;
            }
            if self.peek() == '\n' {
                self.current_line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Token::new(
                TokenType::Error,
                "Unterminated string".into(),
                self.start_line,
                self.current_line,
            );
        }
        self.advance();
        self.add_token(TokenType::String)
    }

    fn char_literal(&mut self) -> Token {
        if self.peek() == '\\' {
            self.advance();
            if self.peek() == '\n' {
                self.current_line += 1;
            }
            self.advance();
        } else {
            self.advance();
        }
        if self.is_at_end() || self.peek() != '\'' {
            return Token::new(
                TokenType::Error,
                "Unterminated char literal".into(),
                self.start_line,
                self.current_line,
            );
        }
        self.advance();
        self.add_token(TokenType::Char)
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
            return self.add_token(TokenType::Float);
        }
        self.add_token(TokenType::Int)
    }

    fn identifier(&mut self) -> Token {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let tok_type = self
            .keyword_trie
            .get_token_type(&text)
            .unwrap_or(TokenType::Identifier);
        self.add_token(tok_type)
    }

    fn add_token(&self, token_type: TokenType) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(token_type, lexeme, self.start_line, self.current_line)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        *self.source.get(self.current).unwrap_or(&'\0')
    }

    fn peek_next(&self) -> char {
        *self.source.get(self.current + 1).unwrap_or(&'\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn initialize_keyword_trie() -> TrieNode {
        let mut root = TrieNode::default();
        root.insert("and", TokenType::And);
        root.insert("class", TokenType::Class);
        root.insert("else", TokenType::Else);
        root.insert("false", TokenType::False);
        root.insert("for", TokenType::For);
        root.insert("in", TokenType::In);
        root.insert("fun", TokenType::Fun);
        root.insert("if", TokenType::If);
        root.insert("none", TokenType::None);
        root.insert("or", TokenType::Or);
        root.insert("print", TokenType::Print);
        root.insert("return", TokenType::Return);
        root.insert("super", TokenType::Super);
        root.insert("this", TokenType::This);
        root.insert("true", TokenType::True);
        root.insert("var", TokenType::Var);
        root.insert("while", TokenType::While);
        root.insert("break", TokenType::Break);
        root.insert("continue", TokenType::Continue);
        root.insert("not", TokenType::Not);
        root
    }
}
