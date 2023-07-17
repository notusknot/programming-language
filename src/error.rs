use crate::tokenizer::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct LoxError {
    span: Span,
    message: String,
    error_type: ErrorType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    ParseError,
}

impl LoxError {
    pub fn error(span: Span, message: &str, error_type: ErrorType) -> Self {
        let err = Self {
            span,
            message: message.to_string(),
            error_type,
        };

        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {:?}] Error{}: {}", self.span, loc, self.message);
    }
}
