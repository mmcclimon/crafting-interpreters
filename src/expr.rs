use crate::token::Token;

// This might be totally bananas, but we'll see.

#[derive(Debug)]
pub enum Expr {
  Binary(Box<Expr>, Token, Box<Expr>),
  Grouping(Box<Expr>),
  Literal(Token),
  Unary(Token, Box<Expr>),
}
