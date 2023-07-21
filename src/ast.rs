#![allow(dead_code)]
#![allow(unused_variables)]
use crate::error::*;
use crate::tokens::{Object, Token};

// expressions

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
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> Result<T, LoxError> {
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
    fn visit_literal_expr(&mut self, literal: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&mut self, unary: &UnaryExpr) -> Result<T, LoxError>;
    fn visit_binary_expr(&mut self, binary: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr) -> Result<T, LoxError>;
}

// statements

#[derive(Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var,
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}

pub trait StmtVisitor<T> {
    fn visit_expr(&mut self, expr: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_print(&mut self, expr: &PrintStmt) -> Result<T, LoxError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expr(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print(self)
    }
}
