use lox::expr::{Expr, Literal};
use lox::token::{Token, TokenType as TT};

fn main() {
  let e = Box::new(Expr::Binary(
    Box::new(Expr::Unary(
      Token::new(TT::Minus, "-".into(), 1),
      Box::new(Expr::Literal(Literal::Number(123.0))),
    )),
    Token::new(TT::Star, "*".into(), 1),
    Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
      45.67,
    ))))),
  ));

  println!("{}", to_string(e));
}

fn to_string(expr: Box<Expr>) -> String {
  match *expr {
    Expr::Binary(left, op, right) => parenthesize(&op.lexeme, vec![left, right]),
    Expr::Grouping(e) => parenthesize("group", vec![e]),
    Expr::Unary(op, right) => parenthesize(&op.lexeme, vec![right]),
    Expr::Literal(val) => format!("{val}"),
  }
}

fn parenthesize(name: &str, exprs: Vec<Box<Expr>>) -> String {
  let mut s = String::from("(");
  s.push_str(name);

  for expr in exprs {
    s.push(' ');
    s.push_str(&to_string(expr));
  }

  s.push(')');
  s
}
