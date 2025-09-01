use crate::tokenizer::token_types::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    StringLiteral(String),
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl {
        name: String,
        value: Expr,
    },
    PrintStmt(Expr),
    FunctionStmt {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}
