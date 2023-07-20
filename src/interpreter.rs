pub struct Interpreter;

use crate::{ast::*, tokens::*};

impl Interpreter {
    pub fn new() -> Self {
        Self
    }
    pub fn evaluate(&mut self, expr: &Expr) -> Object {
        expr.accept(self)
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_unary_expr(&mut self, unary: &UnaryExpr) -> Object {
        let right = self.evaluate(&unary.right);
        println!("{right}");

        todo!()
        /*
        match unary.operator.token_type {
            Minus => -right,
        }
        */
    }
    fn visit_literal_expr(&mut self, literal: &LiteralExpr) -> Object {
        todo!();
    }
    fn visit_binary_expr(&mut self, binary: &BinaryExpr) -> Object {
        todo!()
    }
    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr) -> Object {
        todo!()
    }
}
