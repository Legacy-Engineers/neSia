#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Func,
    Extern,
    Delimiter, // ';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}
