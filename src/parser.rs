use crate::error::{ErrorType::ParseError, LoxError};
use crate::expr::{BinaryExpr, Expr, Expr::Literal, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::tokenizer::{
    KeywordType,
    KeywordType::Return,
    Object::{False, Nil, Num, Str, True},
    Token, TokenType,
    TokenType::{
        BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Keyword, LeftParen, Less, LessEqual,
        Minus, Number, Plus, RightParen, Semicolon, Slash, Star, StringLiteral, Whitespace,
    },
};

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

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        // this will be expanded on when statements are added
        match self.expression()? {
            expr => Ok(expr),
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
            let operator = self.previous().unwrap();
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

        while self.is_match(&[Greater, GreaterEqual, Less, LessEqual, EqualEqual]) {
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
            println!("{operator}");
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

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let start = self.peek().unwrap().span.start;
        let end = self.peek().unwrap().span.end;

        match self.peek().unwrap().token_type {
            Keyword(KeywordType::False) => {
                self.advance();
                Ok(Literal(LiteralExpr { value: False }))
            }
            Keyword(KeywordType::True) => {
                self.advance();
                Ok(Literal(LiteralExpr { value: True }))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Literal(LiteralExpr { value: Nil }))
            }
            Whitespace => {
                self.advance();
                Ok(Literal(LiteralExpr { value: Nil }))
            }
            StringLiteral => {
                self.advance();
                Ok(Literal(LiteralExpr {
                    value: Str(self.source[start..end].to_string()),
                }))
            }
            Number => {
                self.advance();
                Ok(Literal(LiteralExpr {
                    value: Num(self.source[start..end].parse::<f64>().unwrap()),
                }))
            }
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression")?;
                Ok(Expr::Grouping(GroupingExpr {
                    expression: Box::new(expr),
                }))
            }
            _ => Err(LoxError::error(
                self.tokens[self.current + 1].span,
                "Expected expression",
                ParseError,
            )),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let _p = self.peek();
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

    fn check(&self, token_type: TokenType) -> bool {
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
        self.peek().unwrap()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).copied()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).copied()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.peek().unwrap().token_type == Semicolon {
                return;
            }

            if self.peek().unwrap().token_type == Keyword(Return) {
                return;
            }

            self.advance();
        }
    }
}
