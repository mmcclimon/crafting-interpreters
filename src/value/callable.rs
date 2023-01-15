use std::sync::Arc;

use crate::value::{Func, LoxValue};
use crate::{Interpreter, Result};

#[derive(Clone)]
pub struct Callable {
  pub name: String,
  pub arity: usize,
  // this Arc is just so that I can implement Clone, which I need to do for Reasons.
  func: Arc<Box<Func>>,
}

impl Callable {
  pub fn new(name: String, arity: usize, func: Box<Func>) -> Callable {
    Callable {
      arity,
      name,
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
    write!(f, "<function {}>", self.name)
  }
}
