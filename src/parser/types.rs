#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl {
        name: String,
        value: Expr,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Print(Expr),
}
