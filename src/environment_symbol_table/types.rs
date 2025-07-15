use crate::environment_symbol_table::Environment;
use crate::parser::parser_types::Stmt;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Rc<Environment>,
}


#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(Function),
    Nil,
}




