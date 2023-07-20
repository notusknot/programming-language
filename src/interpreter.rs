pub struct Interpreter;

use crate::{ast::*, error::*, tokens::*};

impl Interpreter {
    pub fn new() -> Self {
        Self
    }
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_unary_expr(&mut self, unary: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus | TokenType::Bang => match right {
                Object::Num(n) => Ok(Object::Num(-n)),
                Object::Bool(x) => Ok(Object::Bool(!x)),
                // no truthiness or falsiness. hooray!
                _ => Err(LoxError::error(
                    Span::from(0..1),
                    "Cannot negate non-numeric or non-boolean value",
                    ErrorType::RuntimeError,
                )),
            },
            _ => panic!("Something has gone very wrong in the interpreter..."),
        }
    }
    fn visit_literal_expr(&mut self, literal: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(literal.value.clone())
    }
    fn visit_binary_expr(&mut self, binary: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        let left_num = match left {
            Object::Num(n) => n,
            _ => {
                return Err(LoxError::error(
                    Span::from(0..1),
                    "Left operand is not a number",
                    ErrorType::RuntimeError,
                ))
            }
        };

        let right_num = match right {
            Object::Num(n) => n,
            _ => {
                return Err(LoxError::error(
                    Span::from(0..1),
                    "Right operand is not a number",
                    ErrorType::RuntimeError,
                ))
            }
        };

        match binary.operator.token_type {
            TokenType::Plus => Ok(Object::Num(left_num + right_num)),
            TokenType::Minus => Ok(Object::Num(left_num - right_num)),
            TokenType::Star => Ok(Object::Num(left_num * right_num)),
            TokenType::Slash => Ok(Object::Num(left_num / right_num)),
            TokenType::Greater => Ok(Object::Bool(left_num > right_num)),
            TokenType::GreaterEqual => Ok(Object::Bool(left_num >= right_num)),
            TokenType::Less => Ok(Object::Bool(left_num < right_num)),
            TokenType::LessEqual => Ok(Object::Bool(left_num <= right_num)),
            TokenType::EqualEqual => Ok(Object::Bool(left_num == right_num)),
            TokenType::BangEqual => Ok(Object::Bool(left_num != right_num)),
            _ => panic!("Something has gone very wrong in the interpreter..."),
        }
    }
    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr) -> Result<Object, LoxError> {
        Ok(self.evaluate(&grouping.expression)?)
    }
}
