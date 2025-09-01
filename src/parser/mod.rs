pub mod types;

use crate::lexer::types::Token;
use crate::parser::types::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token
    }

    fn consume(&mut self, expected: Token, message: &str) -> Result<(), String> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(message.to_string())
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Func => self.function_decl(),
            Token::Ident(_) => self.var_decl_or_expr(),
            _ => self.statement(),
        }
    }

    fn function_decl(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'func'
        let name = if let Token::Ident(name) = self.advance() {
            name.clone()
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(
            Token::OpeningParenthesis,
            "Expected '(' after function name",
        )?;

        let mut params = vec![];
        if !self.check(&Token::ClosingParenthesis) {
            loop {
                if let Token::Ident(param) = self.advance() {
                    params.push(param.clone());
                } else {
                    return Err("Expected parameter name".to_string());
                }

                if self.check(&Token::ClosingParenthesis) {
                    break;
                }

                self.consume(Token::Comma, "Expected ',' between parameters")?;
            }
        }

        self.consume(Token::ClosingParenthesis, "Expected ')' after parameters")?;
        self.consume(Token::OpeningBrace, "Expected '{' before function body")?;

        let mut body = vec![];
        while !self.check(&Token::ClosingBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }

        self.consume(Token::ClosingBrace, "Expected '}' after function body")?;

        Ok(Stmt::FunctionDecl { name, params, body })
    }

    fn var_decl_or_expr(&mut self) -> Result<Stmt, String> {
        let token = self.advance().clone(); // clone the token to take ownership

        if let Token::Ident(name) = token {
            if self.check(&Token::Operator("=".to_string())) {
                self.advance(); // consume '='
                let value = self.expression()?;
                self.consume(Token::Delimiter, "Expected ';' after variable declaration")?;
                return Ok(Stmt::VarDecl { name, value });
            } else {
                let expr = self.expression()?;
                self.consume(Token::Delimiter, "Expected ';' after expression")?;
                return Ok(Stmt::ExprStmt(expr));
            }
        }

        Err("Invalid declaration or expression".to_string())
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Func => self.function_decl(),
            Token::Ident(name) if name == "print" => {
                self.advance(); // consume 'print'
                self.consume(Token::OpeningParenthesis, "Expected '(' after print")?;
                let expr = self.expression()?;
                self.consume(
                    Token::ClosingParenthesis,
                    "Expected ')' after print argument",
                )?;
                self.consume(Token::Delimiter, "Expected ';' after print statement")?;
                Ok(Stmt::Print(expr))
            }
            _ => {
                let expr = self.expression()?;
                self.consume(Token::Delimiter, "Expected ';' after expression")?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }
    fn expression(&mut self) -> Result<Expr, String> {
        self.binary_expression()
    }

    fn binary_expression(&mut self) -> Result<Expr, String> {
        // Parse primary first (number, identifier, or call)
        let mut expr = self.primary()?;

        // Parse binary operations
        while let Token::Operator(op) = self.peek() {
            let op = op.clone();
            self.advance(); // consume operator
            let right = self.primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance().clone() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Ident(name) => {
                // Check for function call
                if self.check(&Token::OpeningParenthesis) {
                    self.advance(); // consume '('
                    let mut args = vec![];
                    if !self.check(&Token::ClosingParenthesis) {
                        loop {
                            args.push(self.expression()?);
                            if self.check(&Token::ClosingParenthesis) {
                                break;
                            }
                            self.consume(Token::Comma, "Expected ',' between arguments")?;
                        }
                    }
                    self.consume(Token::ClosingParenthesis, "Expected ')' after arguments")?;
                    Ok(Expr::Call {
                        callee: Box::new(Expr::Identifier(name)),
                        arguments: args,
                    })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }
}
