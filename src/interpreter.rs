use crate::{ast::*, environment::*, error::*, tokens::*};

pub struct Interpreter<'a> {
    environment: Environment<'a>,
    source: &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            environment: Environment::new(source),
            source,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                e.report();
                break;
            }
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        // statements dont return anything so we return an empty ok
        stmt.accept(self)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Object> for Interpreter<'_> {
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
        self.evaluate(&grouping.expression)
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Object, LoxError> {
        // TODO: Shouldn't copy
        self.environment.get(expr.name).cloned()
    }
}

impl StmtVisitor<()> for Interpreter<'_> {
    fn visit_expr(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        if stmt.initializer.is_some() {
            let value = Some(self.evaluate(&stmt.initializer.as_ref().unwrap())?);
            println!("{:?}", value);
            self.environment
                .define(stmt.name.as_string(&self.source), value);
            Ok(())
        } else {
            return Err(LoxError::error(
                stmt.name.span,
                "Cannot use uninitialized value",
                ErrorType::RuntimeError,
            ));
        }
    }
}
