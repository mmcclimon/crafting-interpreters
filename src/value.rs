use std::sync::Arc;

use crate::expr::Literal;
use crate::interpreter::Interpreter;
use crate::{Error, Result};

// This is framework I suspect I will need, but am shoving in here for
// expediency and I'll move it later.
#[derive(Debug, Clone)]
pub enum LoxValue {
  Number(f64),
  String(String),
  Boolean(bool),
  Function(Box<Callable>),
  Nil,
}

pub type Func = dyn Fn(&mut Interpreter, Vec<LoxValue>) -> Result<LoxValue>;

#[derive(Clone)]
pub struct Callable {
  pub arity: usize,
  // this Arc is just so that I can implement Clone, which I need to do for Reasons.
  func: Arc<Box<Func>>,
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
      panic!("tried to call as_number() on a non-number variant");
    }
  }

  pub fn is_callable(&self) -> bool {
    match self {
      Self::Function(_) => true,
      _ => false,
    }
  }

  pub fn as_callable(&self) -> &Callable {
    match self {
      Self::Function(c) => c,
      _ => panic!("tried to call as_callable on a non-callable variant"),
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
      LoxValue::Function(_) => write!(f, "[function object]"),
      LoxValue::Nil => write!(f, "nil"),
    }
  }
}

impl std::cmp::PartialEq for LoxValue {
  fn eq(&self, other: &LoxValue) -> bool {
    use LoxValue as LV;

    match (self, other) {
      (LV::Number(a), LV::Number(b)) => a == b,
      (LV::String(a), LV::String(b)) => a == b,
      (LV::Boolean(a), LV::Boolean(b)) => a == b,
      (LV::Function(_), LV::Function(_)) => false, // functions are never equal
      (LV::Nil, LV::Nil) => true,
      _ => false,
    }
  }
}

impl Callable {
  pub fn new(func: Box<Func>, arity: usize) -> Callable {
    Callable {
      arity,
      func: Arc::new(func),
    }
  }

  pub fn call(
    &self,
    interp: &mut Interpreter,
    args: Vec<LoxValue>,
  ) -> Result<LoxValue> {
    (self.func)(interp, args)
  }
}

impl std::fmt::Debug for Callable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<native function>")
  }
}
