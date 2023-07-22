use crate::ast::{Stmt::Var, *};
use crate::error::{ErrorType::ParseError, LoxError};
use crate::scanner::Scanner;
use crate::tokens::{TokenType::*, *};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'source> {
    tokens: Vec<Token>,
    source: &'source str,
    current: usize,
}

impl<'source> Parser<'source> {
    pub fn new(source: &str, tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            source,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        // this will be expanded on when statements are added
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
        /*
        match self.expression()? {
            expr => Ok(expr),
            _ => Err(LoxError::error(
                self.tokens[self.current].span,
                "Need either statement or expression (how did you even get here?)",
                ParseError,
            )),
        }
        */
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if let Some(token) = self.is_match(&[Keyword(KeywordType::Print)]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;

        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        if let Some(operator) = self.is_match(&[BangEqual, Equal]) {
            println!("{}", operator);
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        if let Some(operator) = self.is_match(&[Greater, GreaterEqual, Less, LessEqual, EqualEqual])
        {
            let right = self.term()?;
            println!("{}", operator);
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        if let Some(operator) = self.is_match(&[Plus, Minus]) {
            println!("{}", operator);
            let right = self.factor()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        if let Some(operator) = self.is_match(&[Slash, Star]) {
            println!("{}", operator);
            let right = self.unary()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if let Some(operator) = self.is_match(&[Bang, Minus]) {
            println!("{}", operator);
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let start = self.peek().span.start;
        let end = self.peek().span.end;

        let token_type = self.peek().token_type;

        let value = match token_type {
            Keyword(KeywordType::False) => Object::Bool(false),
            Keyword(KeywordType::True) => Object::Bool(true),
            Keyword(KeywordType::Nil) => Object::Nil,
            // cut out the quotes
            StringLiteral => Object::Str(self.source[start + 1..end - 1].to_string()),
            Number => Object::Num(self.source[start..end].parse::<f64>().unwrap()),
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression")?;
                return Ok(Expr::Grouping(GroupingExpr {
                    expression: Box::new(expr),
                }));
            }
            _ => {
                return Err(LoxError::error(
                    self.tokens[self.current].span,
                    "Expected expression",
                    ParseError,
                ));
            }
        };
        if !self.is_at_end() {
            self.advance();
        }
        Ok(Expr::Literal(LiteralExpr { value }))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::error(
                self.tokens[self.current - 1].span,
                message,
                ParseError,
            ))
        }
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for &t in token_types {
            if self.check(t) {
                self.advance();
                return Some(self.peek());
            }
        }

        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).copied().unwrap()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).copied().unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.tokens.get(self.current).is_none()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.peek().token_type == Semicolon {
                return;
            }

            if self.peek().token_type == Keyword(KeywordType::Return) {
                return;
            }

            self.advance();
        }
    }
}
