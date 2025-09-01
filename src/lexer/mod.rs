pub mod types;
use crate::lexer::types::Token;

pub struct Tokenizer {
    source: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                c if c.is_whitespace() => {
                    self.advance(); // skip
                }
                c if c.is_alphabetic() => {
                    tokens.push(self.consume_identifier());
                }
                c if c.is_numeric() => {
                    let number = self.consume_number();
                    tokens.push(Token::Number(number));
                }
                '"' => {
                    let string = self.consume_string();
                    tokens.push(Token::Ident(string));
                }
                ';' => {
                    self.advance();
                    tokens.push(Token::Delimiter);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::OpeningParenthesis);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::ClosingParenthesis);
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::Comma);
                }
                '{' => {
                    self.advance();
                    tokens.push(Token::OpeningBrace);
                }
                '}' => {
                    self.advance();
                    tokens.push(Token::ClosingBrace);
                }
                '=' | '+' | '-' | '*' | '/' => {
                    let op = self.advance().unwrap().to_string();
                    tokens.push(Token::Operator(op));
                }
                _ => {
                    println!("Unhandled char: {}", ch);
                    self.advance();
                }
            }
        }

        tokens
    }

    fn consume_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "func" => Token::Func,
            "extern" => Token::Extern,
            _ => Token::Ident(ident),
        }
    }

    fn consume_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse::<f64>().unwrap()
    }

    fn consume_string(&mut self) -> String {
        self.advance(); // skip opening "
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // skip closing "
                break;
            } else {
                result.push(c);
                self.advance();
            }
        }
        result
    }
}
