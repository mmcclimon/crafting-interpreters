use std::collections::HashMap;

use crate::value::LoxValue;
use crate::{Error, Result, Token};

#[derive(Debug)]
pub struct Environment {
  values: HashMap<String, LoxValue>,
}

impl Environment {
  pub fn new() -> Self {
    Environment {
      values: HashMap::new(),
    }
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn assign(&mut self, tok: &Token, new_value: LoxValue) -> Result<()> {
    let name = tok.lexeme();

    if let Some(val) = self.values.get_mut(&name) {
      *val = new_value;
      Ok(())
    } else {
      Err(Error::Runtime(
        tok.clone(),
        format!("undefined variable '{name}'."),
      ))
    }
  }

  pub fn get(&self, tok: &Token) -> Result<LoxValue> {
    let name = tok.lexeme();

    if let Some(val) = self.values.get(&name) {
      Ok((*val).clone())
    } else {
      Err(Error::Runtime(
        tok.clone(),
        format!("undefined variable '{name}'."),
      ))
    }
  }
}
