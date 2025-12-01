use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    FStringLiteral(String), // Raw f-string with {} placeholders
    BoolLiteral(bool),

    // Identifiers and keywords
    Identifier(String),
    Def,
    Class,
    Import,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Return,
    Pass,
    Break,
    Continue,
    And,
    Or,
    Not,
    True,
    False,
    None,

    // Types
    IntType,
    FloatType,
    BoolType,
    StrType,
    ListType,
    DictType,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleSlash,
    DoubleStar,
    Equal,
    DoubleEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Arrow,
    Dot,

    // Special
    Newline,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && self.peek(1).map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::FloatLiteral(num_str.parse().unwrap())
        } else {
            Token::IntLiteral(num_str.parse().unwrap())
        }
    }

    fn read_string(&mut self, quote: char) -> Token {
        let mut string = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    let escaped_char = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        _ => escaped,
                    };
                    string.push(escaped_char);
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Token::StringLiteral(string)
    }

    fn read_fstring(&mut self, quote: char) -> Token {
        let mut string = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    let escaped_char = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        '{' => '{',
                        '}' => '}',
                        _ => escaped,
                    };
                    string.push(escaped_char);
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Token::FStringLiteral(string)
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "def" => Token::Def,
            "class" => Token::Class,
            "import" => Token::Import,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return,
            "pass" => Token::Pass,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "True" => Token::True,
            "False" => Token::False,
            "None" => Token::None,
            "int" => Token::IntType,
            "float" => Token::FloatType,
            "bool" => Token::BoolType,
            "str" => Token::StrType,
            "list" => Token::ListType,
            "dict" => Token::DictType,
            _ => Token::Identifier(ident),
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            if self.current_char == Some('#') {
                self.skip_comment();
                continue;
            }

            match self.current_char {
                None => return Token::Eof,
                Some('\n') => {
                    self.advance();
                    return Token::Newline;
                }
                Some(ch) if ch.is_ascii_digit() => return self.read_number(),
                Some('f') => {
                    // Check if this is an f-string
                    if self.position + 1 < self.input.len() {
                        let next_char = self.input[self.position + 1];
                        if next_char == '"' || next_char == '\'' {
                            self.advance(); // skip 'f'
                            return self.read_fstring(next_char);
                        }
                    }
                    // Otherwise it's just an identifier
                    return self.read_identifier();
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => return self.read_identifier(),
                Some('"') => return self.read_string('"'),
                Some('\'') => return self.read_string('\''),
                Some('+') => {
                    self.advance();
                    return Token::Plus;
                }
                Some('-') => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        return Token::Arrow;
                    }
                    return Token::Minus;
                }
                Some('*') => {
                    self.advance();
                    if self.current_char == Some('*') {
                        self.advance();
                        return Token::DoubleStar;
                    }
                    return Token::Star;
                }
                Some('/') => {
                    self.advance();
                    if self.current_char == Some('/') {
                        self.advance();
                        return Token::DoubleSlash;
                    }
                    return Token::Slash;
                }
                Some('%') => {
                    self.advance();
                    return Token::Percent;
                }
                Some('=') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::DoubleEqual;
                    }
                    return Token::Equal;
                }
                Some('!') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::NotEqual;
                    }
                    panic!("Unexpected character: !");
                }
                Some('<') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::LessEqual;
                    }
                    return Token::Less;
                }
                Some('>') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::GreaterEqual;
                    }
                    return Token::Greater;
                }
                Some('(') => {
                    self.advance();
                    return Token::LeftParen;
                }
                Some(')') => {
                    self.advance();
                    return Token::RightParen;
                }
                Some('{') => {
                    self.advance();
                    return Token::LeftBrace;
                }
                Some('}') => {
                    self.advance();
                    return Token::RightBrace;
                }
                Some('[') => {
                    self.advance();
                    return Token::LeftBracket;
                }
                Some(']') => {
                    self.advance();
                    return Token::RightBracket;
                }
                Some(',') => {
                    self.advance();
                    return Token::Comma;
                }
                Some(':') => {
                    self.advance();
                    return Token::Colon;
                }
                Some(';') => {
                    self.advance();
                    return Token::Semicolon;
                }
                Some('.') => {
                    self.advance();
                    return Token::Dot;
                }
                Some(ch) => {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
