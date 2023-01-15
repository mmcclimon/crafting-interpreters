use std::sync::Arc;

use crate::value::LoxValue;
use crate::{Interpreter, Result};

pub type Func = dyn Fn(&mut Interpreter, Vec<LoxValue>) -> Result<LoxValue>;

#[derive(Clone)]
pub struct Callable {
  pub arity: usize,
  // this Arc is just so that I can implement Clone, which I need to do for Reasons.
  func: Arc<Box<Func>>,
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
