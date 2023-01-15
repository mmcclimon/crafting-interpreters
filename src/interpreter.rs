mod globals;

use crate::environment::Environment;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::value::LoxValue;
use crate::{Error, Result, Token, TokenType as TT};

#[derive(Debug)]
pub struct Interpreter {
  env: Environment,
}

impl Interpreter {
  pub fn new() -> Self {
    let mut int = Interpreter {
      env: Environment::new(),
    };

    globals::install_in(&mut int.env);

    int
  }

  pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
    for stmt in statements {
      self.execute(&stmt)?;
    }

    Ok(())
  }

  fn execute(&mut self, stmt: &Stmt) -> Result<()> {
    match stmt {
      Stmt::Empty => (),
      Stmt::Block(block) => {
        self.env.push_scope();
        self.execute_block(block)?;
        self.env.pop_scope();
      },
      Stmt::Expression(e) => {
        self.eval_expr(e)?;
      },
      Stmt::Print(e) => {
        let val = self.eval_expr(e)?;
        println!("{}", val);
      },
      Stmt::Var(name, init) => {
        let value = self.eval_expr(init)?;
        self.env.define(name, value);
      },
      Stmt::Function(name, params, body) => {
        // We have to clone here to appease the borrow checker, because I
        // haven't structured things in such a way that it can tell the func
        // won't outlive the lifetime of our environment.
        let pclone = params.clone();
        let bclone = body.clone();

        let func =
          move |interp: &mut Interpreter, args: Vec<LoxValue>| -> Result<LoxValue> {
            interp.env.push_scope();

            for (idx, param) in pclone.iter().enumerate() {
              interp.env.define(
                &param.lexeme(),
                args.get(idx).expect("arity mistake").clone(),
              );
            }

            interp.execute_block(&bclone)?;

            interp.env.pop_scope();

            Ok(LoxValue::Nil)
          };

        let callable =
          LoxValue::new_callable(name.lexeme(), params.len(), Box::new(func));

        self.env.define(&name.lexeme(), callable);
      },

      // control flow
      Stmt::If(cond, then_branch, else_branch) => {
        if self.eval_expr(cond)?.is_truthy() {
          self.execute(then_branch)?;
        } else {
          self.execute(else_branch)?;
        }
      },

      Stmt::While(cond, body) => {
        // This cloning sorta stinks, but I wrote this such that eval consumes
        // the expr. I should reconsider that, maybe, but it wasn't trivially
        // doable, so let's get this working first.
        while self.eval_expr(cond)?.is_truthy() {
          self.execute(body)?;
        }
      },
    };

    Ok(())
  }

  fn execute_block(&mut self, block: &Vec<Stmt>) -> Result<()> {
    // Possibly, I should do something to restore the scope if executing the
    // statement fails, but in reality we're going to propogate that all the
    // way up the stack and tear down anyway, so let's just not bother for
    // now.
    for statement in block {
      self.execute(statement)?;
    }

    Ok(())
  }

  fn eval_expr(&mut self, expr: &Box<Expr>) -> Result<LoxValue> {
    let val = match expr.as_ref() {
      Expr::Literal(val) => val.clone().into(),
      Expr::Grouping(e) => self.eval_expr(&e)?,
      Expr::Unary(ref op, ref right) => self.eval_unary_expr(op, right)?,
      Expr::Binary(ref left, ref op, ref right) => {
        self.eval_binary_expr(left, op, right)?
      },
      Expr::Variable(ref token) => self.env.get(token)?,
      Expr::Assign(token, expr) => {
        let value = self.eval_expr(&expr)?;
        self.env.assign(&token, value.clone())?;
        value
      },
      Expr::Logical(left, op, right) => {
        let left_val = self.eval_expr(&left)?;
        let left_true = left_val.is_truthy();

        if op.kind_matches(&TT::Or) {
          if left_true {
            left_val
          } else {
            self.eval_expr(&right)?
          }
        } else {
          if left_true {
            self.eval_expr(&right)?
          } else {
            left_val
          }
        }
      },
      Expr::Call(callee, paren, args) => {
        let callee = self.eval_expr(callee)?;

        if !callee.is_callable() {
          return Err(Error::Runtime(
            paren.clone(),
            "can only call functions and classes".into(),
          ));
        }

        let func = callee.as_callable();
        if args.len() != func.arity {
          return Err(Error::Runtime(
            paren.clone(),
            format!("Expected {} arguments but got {}.", func.arity, args.len()),
          ));
        }

        let mut arguments = vec![];
        for arg in args {
          arguments.push(self.eval_expr(arg)?);
        }

        func.call(self, arguments)?
      },
    };

    Ok(val)
  }

  fn eval_unary_expr(&mut self, op: &Token, right: &Box<Expr>) -> Result<LoxValue> {
    let right = self.eval_expr(right)?;

    match op.kind {
      TT::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
      TT::Minus => {
        if let LoxValue::Number(n) = right {
          Ok(LoxValue::Number(-1.0 * n))
        } else {
          Err(Error::Runtime(
            op.clone(),
            "unary minus only applicable to numbers".into(),
          ))
        }
      },
      _ => unreachable!("bad unary"),
    }
  }

  fn eval_binary_expr(
    &mut self,
    left: &Box<Expr>,
    op: &Token,
    right: &Box<Expr>,
  ) -> Result<LoxValue> {
    use LoxValue as LV;

    let left = self.eval_expr(left)?;
    let right = self.eval_expr(right)?;

    let val = match op.kind {
      // can take any two types
      TT::EqualEqual => LV::Boolean(left.eq(&right)),
      TT::BangEqual => LV::Boolean(!left.eq(&right)),

      // need numbers
      TT::Minus => {
        assert_two_numbers(op, &left, &right)?;
        LV::Number(left.as_number() - right.as_number())
      },
      TT::Slash => {
        assert_two_numbers(op, &left, &right)?;
        LV::Number(left.as_number() / right.as_number())
      },
      TT::Star => {
        assert_two_numbers(op, &left, &right)?;
        LV::Number(left.as_number() * right.as_number())
      },

      // plus is overloaded, to work on strings or numbers
      TT::Plus => match (left, right) {
        (LV::Number(a), LV::Number(b)) => LV::Number(a + b),
        (LV::String(a), LV::String(b)) => LV::String(a + &b),
        _ => {
          return Err(Error::Runtime(
            op.clone(),
            "+ needs either strings or numbers".into(),
          ))
        },
      },

      // numbers, though I think maybe they should work on strings too.
      TT::Greater => {
        assert_two_numbers(op, &left, &right)?;
        LV::Boolean(left.as_number() > right.as_number())
      },
      TT::GreaterEqual => {
        assert_two_numbers(op, &left, &right)?;
        LV::Boolean(left.as_number() >= right.as_number())
      },
      TT::Less => {
        assert_two_numbers(op, &left, &right)?;
        LV::Boolean(left.as_number() < right.as_number())
      },
      TT::LessEqual => {
        assert_two_numbers(op, &left, &right)?;
        LV::Boolean(left.as_number() <= right.as_number())
      },
      _ => unreachable!(),
    };

    Ok(val)
  }
}

fn assert_two_numbers(op: &Token, left: &LoxValue, right: &LoxValue) -> Result<()> {
  if left.type_matches(right) && left.is_number() {
    Ok(())
  } else {
    Err(Error::Runtime(
      op.clone(),
      "operands must be two numbers".into(),
    ))
  }
}
