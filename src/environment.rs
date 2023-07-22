use crate::error::{ErrorType, LoxError};
use crate::tokens::{Object, Token};
use std::collections::HashMap;

pub struct Environment<'source> {
    values: HashMap<String, Object>,
    source: &'source str,
}

impl<'source> Environment<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            values: HashMap::new(),
            source,
        }
    }

    pub fn define(&mut self, name: String, value: Option<Object>) {
        // this makes it so variable statements can redefine variables
        self.values.insert(name, value.unwrap());
    }

    pub fn get(&mut self, name: Token) -> Result<&Object, LoxError> {
        if self.values.contains_key(&name.as_string(&self.source)) {
            println!("{:?}", name);
            return Ok(self.values.get(&name.as_string(&self.source)).unwrap());
        }

        Err(LoxError::error(
            name.span,
            &format!("Undefined variable {}", name),
            ErrorType::RuntimeError,
        ))
    }
}
