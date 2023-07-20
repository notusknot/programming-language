#![allow(dead_code)]
#![allow(unused_variables)]
use crate::tokens::{Object, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Object,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Expr {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        use Expr::*;

        match self {
            Literal(args) => visitor.visit_literal_expr(args),
            Unary(args) => visitor.visit_unary_expr(args),
            Binary(args) => visitor.visit_binary_expr(args),
            Grouping(args) => visitor.visit_grouping_expr(args),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, literal: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, unary: &UnaryExpr) -> T;
    fn visit_binary_expr(&mut self, binary: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr) -> T;
}
