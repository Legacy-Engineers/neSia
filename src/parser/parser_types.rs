use crate::tokenizer::token_types::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
}


#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl {
        name: String,
        value: Expr,
    },
}


