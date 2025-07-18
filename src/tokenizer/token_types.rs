#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equal,
    EqualEqual,   // ==
    BangEqual,    // !=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    Bang,         // !
    Semicolon,
    EOF,
    Print,
    Function,
    Class,
    Let,
    LBrace,
    RBrace,
    Comma,

}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}
