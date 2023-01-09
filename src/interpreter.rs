use crate::expr::{Expr, Literal};
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
      TT::Minus => Ok(LoxValue::Number(-1.0 * right.as_number()?)),
      TT::Bang => Ok(LoxValue::Boolean(!right.is_truthy())),
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

    // TODO improve this error handling
    let val = match op.kind {
      TT::Minus => LV::Number(left.as_number()? - right.as_number()?),
      TT::Slash => LV::Number(left.as_number()? / right.as_number()?),
      TT::Star => LV::Number(left.as_number()? * right.as_number()?),
      TT::Plus => match (left, right) {
        // plus is overloaded, to work on strings or numbers
        (LV::Number(a), LV::Number(b)) => LV::Number(a + b),
        (LV::String(a), LV::String(b)) => LV::String(a + &b),
        _ => return Err(Error::Runtime("+ needs either strings or numbers".into())),
      },
      TT::Greater => LV::Boolean(left.as_number()? > right.as_number()?),
      TT::GreaterEqual => LV::Boolean(left.as_number()? >= right.as_number()?),
      TT::Less => LV::Boolean(left.as_number()? < right.as_number()?),
      TT::LessEqual => LV::Boolean(left.as_number()? <= right.as_number()?),
      TT::BangEqual => LV::Boolean(!left.eq(&right)),
      TT::EqualEqual => LV::Boolean(left.eq(&right)),
      _ => unreachable!(),
    };

    Ok(val)
  }
}

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

  pub fn as_number(&self) -> Result<f64> {
    if let Self::Number(n) = self {
      Ok(*n)
    } else {
      Err(Error::Runtime("called as_number() on a non-number".into()))
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
