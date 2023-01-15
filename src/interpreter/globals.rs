use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::value::{Callable, LoxValue};
use crate::Result;

pub fn install_in(env: &mut Environment) {
  env.define("clock", clock());
}

fn clock() -> LoxValue {
  use std::time::SystemTime;

  let func = |_interp: &mut Interpreter, _args: Vec<LoxValue>| -> Result<LoxValue> {
    let secs = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => LoxValue::Number(n.as_secs() as f64),
      Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    Ok(secs)
  };

  let callable = Callable::new(Box::new(func), 0);
  LoxValue::Function(Box::new(callable))
}
