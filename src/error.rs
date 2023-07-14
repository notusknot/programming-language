#[derive(Debug, PartialEq)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.to_string(),
        }
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}
