use crate::expr::Expr;
use crate::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
  Empty,
  Block(Vec<Stmt>),
  Expression(Box<Expr>),
  Function(Token, Vec<Token>, Vec<Stmt>),
  If(Box<Expr>, Box<Stmt>, Box<Stmt>),
  Print(Box<Expr>),
  Return(Token, Box<Expr>),
  Var(String, Box<Expr>), // maybe instead, Option<Expr>
  While(Box<Expr>, Box<Stmt>),
}
