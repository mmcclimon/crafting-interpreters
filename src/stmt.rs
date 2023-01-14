use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
  Print(Box<Expr>),
  Expression(Box<Expr>),
}
