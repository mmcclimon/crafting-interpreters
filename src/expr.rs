use crate::Token;

// This might be totally bananas, but we'll see.

#[derive(Debug, Clone)]
pub enum Literal {
  Number(f64),
  String(String),
  Boolean(bool),
  Nil,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Assign(Token, Box<Expr>),
  Binary(Box<Expr>, Token, Box<Expr>),
  Grouping(Box<Expr>),
  Literal(Literal),
  Logical(Box<Expr>, Token, Box<Expr>),
  Unary(Token, Box<Expr>),
  Variable(Token),
}

// I just want something to be able to stick in to get stuff to compile while
// I'm working ont it
pub fn nil_expression() -> Box<Expr> {
  Box::new(Expr::Literal(Literal::Nil))
}

pub fn bool_expression(which: bool) -> Box<Expr> {
  Box::new(Expr::Literal(Literal::Boolean(which)))
}

impl std::fmt::Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Literal::Number(n) => write!(f, "{}", n),
      Literal::String(s) => write!(f, "{}", s),
      Literal::Boolean(b) => write!(f, "{}", b),
      Literal::Nil => write!(f, "nil"),
    }
  }
}
