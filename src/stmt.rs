use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
  Block(Vec<Stmt>),
  Expression(Box<Expr>),
  Print(Box<Expr>),
  Var(String, Box<Expr>), // maybe instead, Option<Expr>
}
