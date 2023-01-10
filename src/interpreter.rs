use crate::expr::Expr;
use crate::value::LoxValue;
use crate::{Error, Result, Token, TokenType as TT};

pub struct Interpreter {}

impl Interpreter {
  pub fn interpret(&self, expr: Box<Expr>) -> Result<LoxValue> {
    let val = self.eval(expr)?;
    println!("{:?}", val);

    Ok(val)
  }

  fn eval(&self, expr: Box<Expr>) -> Result<LoxValue> {
    let val = match *expr {
      Expr::Literal(val) => val.into(),
      Expr::Grouping(e) => self.eval(e)?,
      Expr::Unary(op, right) => self.eval_unary(op, right)?,
      Expr::Binary(left, op, right) => self.eval_binary(left, op, right)?,
    };

    Ok(val)
  }

  fn eval_unary(&self, op: Token, right: Box<Expr>) -> Result<LoxValue> {
    let right = self.eval(right)?;

    match op.kind {
      TT::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
      TT::Minus => {
        if let LoxValue::Number(n) = right {
          Ok(LoxValue::Number(-1.0 * n))
        } else {
          Err(Error::Runtime(
            op,
            "unary minus only applicable to numbers".into(),
          ))
        }
      },
      _ => unreachable!("bad unary"),
    }
  }

  fn eval_binary(
    &self,
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
  ) -> Result<LoxValue> {
    use LoxValue as LV;

    let left = self.eval(left)?;
    let right = self.eval(right)?;

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
            op,
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

fn assert_two_numbers(op: Token, left: &LoxValue, right: &LoxValue) -> Result<()> {
  if left.type_matches(right) && left.is_number() {
    Ok(())
  } else {
    Err(Error::Runtime(op, "operands must be two numbers".into()))
  }
}
