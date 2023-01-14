use crate::expr::Literal;
use crate::{Error, Result};

// This is framework I suspect I will need, but am shoving in here for
// expediency and I'll move it later.
#[derive(Debug, PartialEq)]
pub enum LoxValue {
  Number(f64),
  String(String),
  Boolean(bool),
  Nil,
}

impl LoxValue {
  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Nil => false,
      Self::Boolean(val) => *val,
      _ => true,
    }
  }

  pub fn is_number(&self) -> bool {
    if let Self::Number(_) = self {
      true
    } else {
      false
    }
  }

  pub fn as_number(&self) -> f64 {
    if let Self::Number(n) = self {
      *n
    } else {
      panic!("tried to call as_number() on a non-number variiant");
    }
  }

  pub fn type_matches(&self, other: &Self) -> bool {
    use std::mem::discriminant;
    discriminant(self) == discriminant(other)
  }
}

impl From<Literal> for LoxValue {
  fn from(lit: Literal) -> Self {
    match lit {
      Literal::Number(n) => LoxValue::Number(n),
      Literal::String(s) => LoxValue::String(s),
      Literal::Boolean(b) => LoxValue::Boolean(b),
      Literal::Nil => LoxValue::Nil,
    }
  }
}

impl TryFrom<LoxValue> for f64 {
  type Error = Error;

  fn try_from(lv: LoxValue) -> Result<f64> {
    if let LoxValue::Number(n) = lv {
      Ok(n)
    } else {
      Err(Error::TryFrom(format!("{:?} is not a number", lv)))
    }
  }
}

impl std::fmt::Display for LoxValue {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      LoxValue::Number(n) => write!(f, "{}", n),
      LoxValue::String(s) => write!(f, "{}", s),
      LoxValue::Boolean(b) => write!(f, "{}", b),
      LoxValue::Nil => write!(f, "nil"),
    }
  }
}
