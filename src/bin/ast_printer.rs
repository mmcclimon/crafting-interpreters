use lox::expr::Expr;
use lox::token::{Token, TokenType as TT};

fn main() {
  let e = Box::new(Expr::Binary(
    Box::new(Expr::Unary(
      Token::new(TT::Minus, "-".into(), 1),
      Box::new(Expr::Literal(Token::new(
        TT::Number(123.0),
        "123".into(),
        1,
      ))),
    )),
    Token::new(TT::Star, "*".into(), 1),
    Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(
      TT::Number(45.67),
      "45.67".into(),
      1,
    ))))),
  ));

  println!("{}", to_string(e));
}

fn to_string(expr: Box<Expr>) -> String {
  match *expr {
    Expr::Binary(left, op, right) => parenthesize(&op.lexeme, vec![left, right]),
    Expr::Grouping(e) => parenthesize("group", vec![e]),
    Expr::Unary(op, right) => parenthesize(&op.lexeme, vec![right]),
    Expr::Literal(val) => {
      if let Some(string) = val.string_value() {
        string.into()
      } else if let Some(n) = val.numeric_value() {
        format!("{n}")
      } else {
        unreachable!("bogus literal");
      }
    },
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
