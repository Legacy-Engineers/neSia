pub mod token_types;

use token_types::Token;
use token_types::TokenType;

pub struct Tokenizer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.current();
        if let Some(c) = current {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        current
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos + 1).copied()
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let c: char = self.current().unwrap();
            let start_col: usize = self.col;

            match c {
                '0'..='9' => {
                    tokens.push(self.number_token());
                }
                '+' => {
                    tokens.push(self.make_simple_token(TokenType::Plus));
                    self.advance();
                }
                '-' => {
                    tokens.push(self.make_simple_token(TokenType::Minus));
                    self.advance();
                }
                '*' => {
                    tokens.push(self.make_simple_token(TokenType::Star));
                    self.advance();
                }
                '/' => {
                    tokens.push(self.make_simple_token(TokenType::Slash));
                    self.advance();
                }
                '(' => {
                    tokens.push(self.make_simple_token(TokenType::LParen));
                    self.advance();
                }
                ')' => {
                    tokens.push(self.make_simple_token(TokenType::RParen));
                    self.advance();
                }
                ';' => {
                    tokens.push(self.make_simple_token(TokenType::Semicolon));
                    self.advance();
                }
                '{' => {
                    tokens.push(self.make_simple_token(TokenType::LBrace));
                    self.advance();
                }
                '}' => {
                    tokens.push(self.make_simple_token(TokenType::RBrace));
                    self.advance();
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(self.make_simple_token(TokenType::EqualEqual));
                    } else {
                        tokens.push(self.make_simple_token(TokenType::Equal));
                        self.advance();
                    }
                }
                '"' => {
                    tokens.push(self.string_token());
                    self.advance();
                }
                ',' => tokens.push(self.make_simple_token(TokenType::Comma)),

                '"' | '\'' => {
                    tokens.push(self.string_token());
                }

                c if c.is_whitespace() => {
                    self.advance();
                }
                c if c.is_alphabetic() || c == '_' => {
                    tokens.push(self.identifier_token());
                }
                _ => {
                    println!(
                        "Unexpected character '{}' at line {}, col {}",
                        c,
                        self.line,
                        start_col
                    );
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line,
            column: self.col,
        });

        tokens
    }

    fn make_simple_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            column: self.col,
        }
    }

    fn number_token(&mut self) -> Token {
        let start = self.pos;
        let start_col = self.col;

        while let Some(c) = self.current() {
            if c.is_numeric() || c == '.' {
                self.advance();
            } else {
                break;
            }
        }

        let value: f64 = self.src[start..self.pos].iter().collect::<String>().parse().unwrap();

        Token {
            token_type: TokenType::Number(value),
            line: self.line,
            column: start_col,
        }
    }

    fn string_token(&mut self) -> Token {
        let quote_char = self.current().unwrap(); // either " or '
        self.advance(); // skip opening quote

        let start_col = self.col;
        let mut value = String::new();

        while let Some(c) = self.current() {
            if c == quote_char {
                break;
            } else {
                value.push(c);
                self.advance();
            }
        }

        if self.current() != Some(quote_char) {
            eprintln!("Unterminated string at line {}, col {}", self.line, self.col);
        }

        self.advance(); // skip closing quote

        Token {
            token_type: TokenType::StringLiteral(value),
            line: self.line,
            column: start_col,
        }
    }

    fn identifier_token(&mut self) -> Token {
        let start = self.pos;
        let start_col = self.col;

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident = self.src[start..self.pos].iter().collect::<String>();
        let token_type = match ident.as_str() {
            "func" => TokenType::Function,
            "print" => TokenType::Print,
            "class" => TokenType::Class,
            "let" => TokenType::Let,
            _ => TokenType::Identifier(ident),
        };

        Token {
            token_type,
            line: self.line,
            column: start_col,
        }
    }
}
