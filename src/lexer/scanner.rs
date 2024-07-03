use super::token::{Token, TokenType};
use super::helper::TrieNode;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    start_line: i32,    // Track the starting line of the current token
    current_line: i32,  // Track the current line number in the source
    keyword_trie: TrieNode,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            start_line: 1,   // Start at line 1
            current_line: 1, // Start at line 1
            keyword_trie: Self::initialize_keyword_trie(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while!self.is_at_end() {
            self.start = self.current;
            self.start_line = self.current_line; // Update start_line before scanning token
            tokens.push(self.scan_token());
        }

        tokens.push(Token {
            token: TokenType::EOF,
            lexeme: "".to_string(),
            start_line: self.current_line, // EOF is at the current line
            end_line: self.current_line,   // EOF is at the current line
        });

        tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.add_token(TokenType::EOF);
        }

        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => {
                if self.match_char('*') {
                    self.add_token(TokenType::StarStar)
                } else {
                    self.add_token(TokenType::Star)
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.add_token(TokenType::SlashSlash)
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '%' => self.add_token(TokenType::Percent),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '&' => self.add_token(TokenType::Ampersand),
            '|' => self.add_token(TokenType::Pipe),
            '^' => self.add_token(TokenType::Caret),
            ' ' | '\r' | '\t' => self.scan_token(), // Skip whitespace
            '\n' => {
                self.current_line += 1; // Increment line number
                //self.advance(); // Consume newline
                self.scan_token() // Continue scanning
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '\'' => self.char(),
            _ => self.add_token(TokenType::Error),
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    pub fn add_token(&self, token_type: TokenType) -> Token {
        Token {
            token: token_type,
            lexeme: self.source[self.start..self.current].iter().collect(),
            start_line: self.start_line,   // Use the stored start_line
            end_line: self.current_line,   // Use the current line
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current]!= expected {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c: char = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.current_line += 1;
                    self.advance();
                }
               '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
                _ => break,
            }
        }
    }

    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    pub fn string(&mut self) -> Token {
        while self.peek()!= '"' &&!self.is_at_end() {
            if self.peek() == '\n' {
                self.current_line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.add_token(TokenType::Error);
        }

        self.advance(); // Consume closing "
        self.add_token(TokenType::String)
    }

    pub fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
            self.add_token(TokenType::Float)
        } else {
            self.add_token(TokenType::Int)
        }
    }

    pub fn char(&mut self) -> Token {
        if self.is_at_end() || self.peek() == '\'' {
            return self.add_token(TokenType::Error);
        }

        self.advance(); // Consume opening '

        if self.is_at_end() || self.peek()!= '\'' {
            return self.add_token(TokenType::Error);
        }

        self.advance(); // Consume closing '
        self.add_token(TokenType::Char)
    }

    pub fn identifier(&mut self) -> Token {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        match self.keyword_trie.get_token_type(&text) {
            Some(token_type) => self.add_token(token_type),
            None => self.add_token(TokenType::Identifier),
        }
    }

    pub fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    pub fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn initialize_keyword_trie() -> TrieNode {
        let mut root = TrieNode::default();

        root.insert("and", TokenType::And);
        root.insert("Class", TokenType::Class);
        root.insert("else", TokenType::Else);
        root.insert("false", TokenType::False);
        root.insert("for", TokenType::For);
        root.insert("fn", TokenType::Fun);
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
        root.insert("not", TokenType::Not);

        root
    }
}
