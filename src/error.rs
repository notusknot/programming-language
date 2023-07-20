use crate::tokens::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoxError {
    span: Span,
    message: String,
    error_type: ErrorType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorType {
    ParseError,
    RuntimeError,
}

impl LoxError {
    pub fn error(span: Span, message: &str, error_type: ErrorType) -> Self {
        Self {
            span,
            message: message.to_string(),
            error_type,
        }
    }

    pub fn report(&self) {
        eprintln!("[line {:?}] Error: {}", self.span, self.message);
    }
}
