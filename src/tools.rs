pub mod ast_printer {
  use crate::expr::Expr;

  pub fn print_ast(expr: Box<Expr>) {
    println!("{}", to_string(expr));
  }

  fn to_string(expr: Box<Expr>) -> String {
    match *expr {
      Expr::Binary(left, op, right) => parenthesize(&op.lexeme(), vec![left, right]),
      Expr::Grouping(e) => parenthesize("group", vec![e]),
      Expr::Unary(op, right) => parenthesize(&op.lexeme(), vec![right]),
      Expr::Literal(val) => format!("{val}"),
      Expr::Variable(name) => format!("var {name}"),
      Expr::Assign(_tok, _expr) => todo!(),
      Expr::Logical(_left, _op, _right) => todo!(),
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
}
