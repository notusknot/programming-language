use crate::ast::Stmt;
use crate::ast::{
    BinaryExpr, Expr, Expr::Literal, ExpressionStmt, GroupingExpr, LiteralExpr, PrintStmt,
    UnaryExpr,
};
use crate::error::{ErrorType::ParseError, LoxError};
use crate::tokens::{
    KeywordType,
    KeywordType::Return,
    Object::{Bool, Nil, Num, Str},
    Token, TokenType,
    TokenType::{
        BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Keyword, LeftParen, Less, LessEqual,
        Minus, Number, Plus, RightParen, Semicolon, Slash, Star, StringLiteral,
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
        if self.is_match(&[Keyword(KeywordType::Print)]) {
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

        while self.is_match(&[BangEqual, Equal]) {
            let operator = self.previous();
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
            let operator = self.previous();
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
            let operator = self.previous();
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
            let operator = self.previous();
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
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let start = self.current_token().span.start;
        let end = self.current_token().span.end;

        let token_type = self.current_token().token_type;

        let value = match token_type {
            Keyword(KeywordType::False) => Bool(false),
            Keyword(KeywordType::True) => Bool(true),
            TokenType::Nil => Nil,
            // cut out the quotes
            StringLiteral => Str(self.source[start + 1..end - 1].to_string()),
            Number => Num(self.source[start..end].parse::<f64>().unwrap()),
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression")?;
                return Ok(Expr::Grouping(GroupingExpr {
                    expression: Box::new(expr),
                }));
            }
            _ => {
                panic!();
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
        Ok(Literal(LiteralExpr { value }))
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
            self.current_token().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn current_token(&self) -> Token {
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
            if self.current_token().token_type == Semicolon {
                return;
            }

            if self.current_token().token_type == Keyword(Return) {
                return;
            }

            self.advance();
        }
    }
}
