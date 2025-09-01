#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Func,
    Extern,
    Delimiter, // ';'
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace, // added
    ClosingBrace, // added
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}
