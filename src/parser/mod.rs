pub mod parser_types;

use crate::tokenizer::token_types::*;
use parser_types::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenType::Equal]) {
            let op = self.previous_token().token_type.clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let op = self.previous_token().token_type.clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let op = self.previous_token().token_type.clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Minus]) {
            let op = self.previous_token().token_type.clone();
            let right = self.unary()?;
            Ok(Expr::Binary {
                left: Box::new(Expr::Number(0.0)), // simulate negation
                op,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match &self.current_token().token_type {
            TokenType::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            TokenType::Identifier(name) => {
                let ident = name.clone();
                self.advance();
                Ok(Expr::Identifier(ident))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::StringLiteral(val)) // <- you'll need to add this to Expr
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect(TokenType::RParen, "Expected ')'")?;
                Ok(expr)
            }
            _ =>
                Err(
                    format!(
                        "Unexpected token {:?} at line {}, col {}",
                        self.current_token().token_type,
                        self.current_token().line,
                        self.current_token().column
                    )
                ),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token().token_type, TokenType::EOF)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::Function]) {
            self.function_decl()
        } else if self.match_token(&[TokenType::Identifier("let".into())]) {
            self.var_decl()
        } else if self.match_token(&[TokenType::Print]) {
            self.print_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::PrintStmt(value))
    }

    fn function_decl(&mut self) -> Result<Stmt, String> {
        // Match `func`
        self.expect(TokenType::Function, "Expected 'func' keyword")?;

        // Function name
        let name = if let TokenType::Identifier(ref name) = self.current_token().token_type {
            name.clone()
        } else {
            return Err("Expected function name".to_string());
        };
        self.advance();

        // Expect opening (
        self.expect(TokenType::LParen, "Expected '(' after function name")?;

        // Parse parameters (if any)
        let mut params = Vec::new();
        if !self.check(&TokenType::RParen) {
            loop {
                if let TokenType::Identifier(ref name) = self.current_token().token_type {
                    params.push(name.clone());
                    self.advance();
                } else {
                    return Err("Expected parameter name".to_string());
                }

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        // ❗ This is the one you may have missed:
        self.expect(TokenType::RParen, "Expected ')' after parameters")?;

        // Expect {
        self.expect(TokenType::LBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }

        self.expect(TokenType::RBrace, "Expected '}' after function body")?;

        Ok(Stmt::FunctionStmt { name, params, body })
    }

    fn var_decl(&mut self) -> Result<Stmt, String> {
        let name = if let TokenType::Identifier(ref name) = self.current_token().token_type {
            name.clone()
        } else {
            return Err("Expected variable name".to_string());
        };
        self.advance();

        self.expect(TokenType::Equal, "Expected '=' after variable name")?;

        let value = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::VarDecl { name, value })
    }

    fn expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::ExprStmt(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&self.tokens.last().unwrap())
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: &TokenType) -> bool {
        &self.current_token().token_type == token_type
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if &self.current_token().token_type == t {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expect(&mut self, expected: TokenType, message: &str) -> Result<(), String> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(
                format!(
                    "{} at line {}, column {}",
                    message,
                    self.current_token().line,
                    self.current_token().column
                )
            )
        }
    }
}
