use std::collections::HashMap;

use crate::value::LoxValue;
use crate::{Error, Result, Token};

type EnvMap = HashMap<String, LoxValue>;

// This is a concession to my inability to think coherently about rust
// lifetimes. We *should* be able to work that out, but I've poked at it for a
// good long while and can't quite get there.
//
// An Environment is a vec of environments in increasing order of specificity,
// so: [global, outer, inner].
#[derive(Debug)]
pub struct Environment {
  scopes: Vec<EnvMap>,
}

impl Environment {
  pub fn new() -> Self {
    Environment {
      scopes: vec![HashMap::new()],
    }
  }

  pub fn push_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  pub fn pop_scope(&mut self) {
    if self.scopes.len() <= 1 {
      panic!("cannot pop last scope!");
    }

    self.scopes.pop();
  }

  pub fn define(&mut self, name: &str, value: LoxValue) {
    self.scopes.last_mut().unwrap().insert(name.into(), value);
  }

  pub fn get(&self, tok: &Token) -> Result<LoxValue> {
    let name = tok.lexeme();

    for scope in self.scopes.iter().rev() {
      if let Some(val) = scope.get(&name) {
        return Ok((*val).clone());
      }
    }

    Err(Error::Runtime(
      tok.clone(),
      format!("undefined variable '{name}'."),
    ))
  }

  pub fn get_at(&self, dist: usize, tok: &Token) -> Result<LoxValue> {
    let name = tok.lexeme();

    let scope = self
      .scopes
      .iter()
      .rev()
      .skip(dist)
      .next()
      .expect("math error in get_at");

    let val = scope
      .get(&name)
      .expect("variable lookup failed, and should not have");

    Ok((*val).clone())
  }

  pub fn assign(&mut self, tok: &Token, new_value: LoxValue) -> Result<()> {
    let name = tok.lexeme();

    for scope in self.scopes.iter_mut().rev() {
      if let Some(val) = scope.get_mut(&name) {
        *val = new_value;
        return Ok(());
      }
    }

    Err(Error::Runtime(
      tok.clone(),
      format!("undefined variable '{name}'."),
    ))
  }

  pub fn assign_at(
    &mut self,
    dist: usize,
    tok: &Token,
    new_value: LoxValue,
  ) -> Result<()> {
    let name = tok.lexeme();

    let scope = self
      .scopes
      .iter_mut()
      .rev()
      .skip(dist)
      .next()
      .expect("math error in get_at");

    scope.insert(name, new_value);

    Ok(())
  }
}
