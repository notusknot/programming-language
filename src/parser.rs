use crate::error::{ErrorType::*, LoxError};
use crate::expr::*;
use crate::tokenizer::{Object, Object::*, Token, TokenType, TokenType::*};

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

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        match self.expression()? {
            Expr => Ok(Expr),
            _ => Err(LoxError::error(
                self.tokens[self.current].span,
                "asdf",
                ParseError,
            )),
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[BangEqual, Equal]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: operator.unwrap(),
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().unwrap();
            let right = self.term()?;
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

        while self.is_match(&[Minus, Plus]) {
            let operator = self.previous().unwrap();
            let right = self.term()?;
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

        while self.is_match(&[Slash, Star]) {
            let operator = self.previous().unwrap();
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
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        // TODO: this shouldn't return nil
        if self.is_match(&[TokenType::Whitespace]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        let start = self.previous().unwrap().span.start;
        let end = self.previous().unwrap().span.end;

        /* TODO: figure out how to get the literal from the span */
        if self.is_match(&[TokenType::Number, TokenType::StringLiteral]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Str(self.source[start..end].to_string())),
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(LoxError::error(
            self.tokens[self.current + 1].span,
            "Expected expression",
            ParseError,
        ))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(LoxError::error(
                self.tokens[self.current].span,
                message,
                ParseError,
            ))
        }
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        for &t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().unwrap().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().unwrap()
    }

    fn previous(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current - 1).copied();
        println!("{:?}", token);
        token
    }

    fn peek(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).copied();
        println!("{:?}", token);
        token
    }

    fn is_at_end(&mut self) -> bool {
        self.peek() == None
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().unwrap().token_type == Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}