use crate::environment_symbol_table::Environment;
use crate::environment_symbol_table::types::{ Value, Function };
use crate::parser::parser_types::{ Expr, Stmt };
use crate::tokenizer::token_types::TokenType;
use std::rc::Rc;

pub struct Interpreter {
    /// Current environment (usually the globals at top-level)
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(Environment::new());
        Self { environment: globals }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        // run everything in the current (global) environment
        let env = Rc::clone(&self.environment);
        for stmt in statements {
            self.execute_stmt(stmt, Rc::clone(&env))?;
        }
        Ok(())
    }

    // ---------------------------
    // Statements
    // ---------------------------

    fn execute_stmt(&mut self, stmt: Stmt, env: Rc<Environment>) -> Result<(), String> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                let _ = self.evaluate_expr(expr, Rc::clone(&env))?;
                Ok(())
            }
            Stmt::PrintStmt(expr) => {
                let val = self.evaluate_expr(expr, Rc::clone(&env))?;
                // Use Display if you implemented it; otherwise Debug is fine:
                println!("{}", display_value(&val));
                Ok(())
            }
            Stmt::FunctionStmt { name, params, body } => {
                // capture the defining environment as the function's closure
                let func = Value::Function(Function {
                    name: name.clone(),
                    params,
                    body,
                });
                env.define(&name, func);
                Ok(())
            }
            // Add other statement kinds here (var, block, if, while, return, etc.)
            _ => Err("Unsupported statement type".to_string()),
        }
    }

    // ---------------------------
    // Expressions
    // ---------------------------

    fn evaluate_expr(&mut self, expr: Expr, env: Rc<Environment>) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::StringLiteral(s) => Ok(Value::String(s)),
            Expr::Identifier(name) => env.get(&name),

            Expr::Binary { left, op, right } => {
                self.evaluate_binary(*left, op, *right, Rc::clone(&env))
            }

            Expr::Call { callee, arguments } => {
                let callee_val = self.evaluate_expr(*callee, Rc::clone(&env))?;
                let mut args = Vec::with_capacity(arguments.len());
                for a in arguments {
                    args.push(self.evaluate_expr(a, Rc::clone(&env))?);
                }
                match callee_val {
                    Value::Function(func) => self.call_function(func, args),
                    _ => Err("Attempted to call a non-function".to_string()),
                }
            }

            // Add more expression kinds as you implement them
            _ => Err("Unsupported expression type".to_string()),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: Expr,
        op: TokenType,
        right: Expr,
        env: Rc<Environment>
    ) -> Result<Value, String> {
        match op {
            // Assignment: identifier = expr
            TokenType::Equal => {
                if let Expr::Identifier(name) = left {
                    let value = self.evaluate_expr(right, Rc::clone(&env))?;
                    env.define(&name, value.clone());
                    Ok(value)
                } else {
                    Err("Invalid assignment target".to_string())
                }
            }

            TokenType::Plus => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
                    _ => Err("Invalid operands for '+'".to_string()),
                }
            }

            TokenType::Minus => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    _ => Err("Invalid operands for '-'".to_string()),
                }
            }

            TokenType::Star => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    _ => Err("Invalid operands for '*'".to_string()),
                }
            }

            TokenType::Slash => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => {
                        if b == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }
                    _ => Err("Invalid operands for '/'".to_string()),
                }
            }

            TokenType::EqualEqual => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                Ok(Value::Boolean(equals(&l, &r)))
            }

            TokenType::BangEqual => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                Ok(Value::Boolean(!equals(&l, &r)))
            }

            TokenType::Greater => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
                    _ => Err("Invalid operands for '>'".to_string()),
                }
            }

            TokenType::GreaterEqual => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
                    _ => Err("Invalid operands for '>='".to_string()),
                }
            }

            TokenType::Less => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
                    _ => Err("Invalid operands for '<'".to_string()),
                }
            }

            TokenType::LessEqual => {
                let l = self.evaluate_expr(left, Rc::clone(&env))?;
                let r = self.evaluate_expr(right, Rc::clone(&env))?;
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
                    _ => Err("Invalid operands for '<='".to_string()),
                }
            }

            _ => Err(format!("Unsupported binary operator: {:?}", op)),
        }
    }

    // ---------------------------
    // Function calls
    // ---------------------------

    fn call_function(&mut self, func: Function, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != func.params.len() {
            return Err(format!("Expected {} arguments but got {}", func.params.len(), args.len()));
        }

        // Create a new environment chained to the function's closure
        let local_env = Environment::with_enclosing(Rc::clone(&self.environment));

        // Bind parameters
        for (param, arg) in func.params.iter().zip(args.into_iter()) {
            local_env.define(param, arg);
        }

        // Execute the function body in the local environment
        for stmt in func.body.iter().cloned() {
            self.execute_stmt(stmt, Rc::clone(&self.environment))?;
            // If/when you add `return`, you'd early-exit here carrying a value.
        }

        Ok(Value::Nil)
    }
}

// ---------------------------
// Helpers
// ---------------------------

fn equals(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::Nil, Value::Nil) => true,
        _ => false,
    }
}

// Pretty print without exposing enum wrappers if you prefer
fn display_value(v: &Value) -> String {
    match v {
        Value::Number(n) => {
            // trim trailing .0 if you like; otherwise just format
            format!("{}", n)
        }
        Value::String(s) => s.clone(),
        Value::Boolean(b) => format!("{}", b),
        Value::Nil => "nil".into(),
        Value::Function(f) => format!("<fn {}>", f.name),
    }
}
