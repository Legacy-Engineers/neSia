use crate::environment_symbol_table::{Environment, types::Value};
use crate::parser::parser_types::{Expr, Stmt};
use crate::tokenizer::token_types::TokenType;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                let result = self.evaluate_expr(expr)?;
                println!("{:?}", result);
                Ok(())
            }
            Stmt::PrintStmt(expr) => {
                let val = self.evaluate_expr(expr)?;
                println!("{:?}", val);
                Ok(())
            }
            _ => Err("Unsupported statement type".to_string()),
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Binary { left, op, right } => {
                self.evaluate_binary(*left, op, *right)
            }
            Expr::Identifier(name) => {
                self.environment.get(&name)
            }
            Expr::Number(value) => {
                Ok(Value::Number(value))
            }
            // Add other expression types as you implement them
            _ => Err("Unsupported expression type".to_string()),
        }
    }

    fn evaluate_binary(&mut self, left: Expr, op: TokenType, right: Expr) -> Result<Value, String> {
        match op {
            TokenType::Equal => {
                // Assignment operation
                if let Expr::Identifier(name) = left {
                    let value = self.evaluate_expr(right)?;
                    self.environment.define(&name, value.clone());
                    Ok(value)
                } else {
                    Err("Invalid assignment target".to_string())
                }
            }
            TokenType::Plus => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                    _ => Err("Invalid operands for addition".to_string()),
                }
            }
            TokenType::Minus => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => Err("Invalid operands for subtraction".to_string()),
                }
            }
            TokenType::Star => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => Err("Invalid operands for multiplication".to_string()),
                }
            }
            TokenType::Slash => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    _ => Err("Invalid operands for division".to_string()),
                }
            }
            TokenType::EqualEqual => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                let result = match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => l == r,
                    (Value::String(l), Value::String(r)) => l == r,
                    (Value::Boolean(l), Value::Boolean(r)) => l == r,
                    (Value::Nil, Value::Nil) => true,
                    _ => false,
                };
                
                Ok(Value::Boolean(result))
            }
            TokenType::BangEqual => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                let result = match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => l != r,
                    (Value::String(l), Value::String(r)) => l != r,
                    (Value::Boolean(l), Value::Boolean(r)) => l != r,
                    (Value::Nil, Value::Nil) => false,
                    _ => true,
                };
                
                Ok(Value::Boolean(result))
            }
            TokenType::Greater => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    _ => Err("Invalid operands for comparison".to_string()),
                }
            }
            TokenType::GreaterEqual => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                    _ => Err("Invalid operands for comparison".to_string()),
                }
            }
            TokenType::Less => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    _ => Err("Invalid operands for comparison".to_string()),
                }
            }
            TokenType::LessEqual => {
                let left_val = self.evaluate_expr(left)?;
                let right_val = self.evaluate_expr(right)?;
                
                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                    _ => Err("Invalid operands for comparison".to_string()),
                }
            }
            _ => Err(format!("Unsupported binary operator: {:?}", op)),
        }
    }
}