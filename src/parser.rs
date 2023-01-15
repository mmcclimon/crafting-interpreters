use std::cell::RefCell;

use crate::expr::{self, Expr, Literal};
use crate::stmt::Stmt;
use crate::{Error, Result, Token, TokenType as TT};

#[derive(Debug)]
pub struct Parser {
  tokens: Vec<Token>,
  current: RefCell<usize>,
  pub errors: Vec<Error>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      current: RefCell::new(0),
      errors: vec![],
    }
  }

  pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
  }

  pub fn parse(&mut self) -> Result<Vec<Stmt>> {
    let mut statements = vec![];

    while !self.is_at_end() {
      let stmt = self.declaration();
      if let Ok(stmt) = stmt {
        statements.push(stmt);
      } else {
        self.errors.push(stmt.unwrap_err());
        self.synchronize();
      }
    }

    if self.errors.is_empty() {
      Ok(statements)
    } else {
      Err(Error::ParseFailed)
    }
  }

  fn declaration(&self) -> Result<Stmt> {
    if self.next_matches(&[TT::Var]) {
      self.var_declaration()
    } else {
      self.statement()
    }
  }

  fn var_declaration(&self) -> Result<Stmt> {
    // this "_" smells here
    let name = self.consume(TT::Identifier("_".into()), "Expect variable name")?;

    let initializer = if self.next_matches(&[TT::Equal]) {
      self.expression()?
    } else {
      expr::nil_expression()
    };

    self.consume(TT::Semicolon, "Expect ';' after variable declaration.")?;
    Ok(Stmt::Var(name.lexeme(), initializer))
  }

  fn statement(&self) -> Result<Stmt> {
    let next = self.peek().expect("no token to parse in statement()");
    self.advance(); // hrm

    let stmt = match next.kind {
      TT::Print => self.print_statement()?,
      TT::LeftBrace => Stmt::Block(self.block()?),
      TT::If => self.if_statement()?,
      TT::While => self.while_statement()?,
      TT::For => self.for_statement()?,
      _ => {
        self.rewind(); // don't actually want that advance after all
        self.expression_statement()?
      },
    };

    Ok(stmt)
  }

  fn print_statement(&self) -> Result<Stmt> {
    let value = self.expression()?;
    self.consume(TT::Semicolon, "Expect ';' after value.")?;
    Ok(Stmt::Print(value))
  }

  fn if_statement(&self) -> Result<Stmt> {
    self.consume(TT::LeftParen, "Expect '(' after 'if'.")?;
    let cond = self.expression()?;
    self.consume(TT::RightParen, "Expect ')' after if condition.")?;

    let then_branch = self.statement()?;
    let else_branch = if self.next_matches(&[TT::Else]) {
      self.statement()?
    } else {
      Stmt::Empty
    };

    Ok(Stmt::If(cond, Box::new(then_branch), Box::new(else_branch)))
  }

  fn while_statement(&self) -> Result<Stmt> {
    self.consume(TT::LeftParen, "Expect '(' after 'while'.")?;
    let cond = self.expression()?;
    self.consume(TT::RightParen, "Expect ')' after while condition.")?;

    let body = self.statement()?;
    Ok(Stmt::While(cond, Box::new(body)))
  }

  // a for statement is just sugar for a while, so this desugars it all
  fn for_statement(&self) -> Result<Stmt> {
    // first parse
    self.consume(TT::LeftParen, "Expect '(' after 'for'.")?;

    let initializer: Option<Stmt> = if self.next_matches(&[TT::Semicolon]) {
      None
    } else if self.next_matches(&[TT::Var]) {
      Some(self.var_declaration()?)
    } else {
      Some(self.expression_statement()?)
    };

    let cond = if !self.check(&TT::Semicolon) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume(TT::Semicolon, "Expect ';' after loop condition.")?;

    let inc = if !self.check(&TT::RightParen) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume(TT::RightParen, "Expect ')' after for clauses.")?;

    let mut body = self.statement()?;

    // now, desugar:
    // desugar the increment onto the end of the block
    if let Some(increment) = inc {
      body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
    }

    // and the condition onto the front of it
    let condition = cond.unwrap_or_else(|| expr::bool_expression(true));
    body = Stmt::While(condition, Box::new(body));

    // and the initializer before the whole thing
    if let Some(init) = initializer {
      body = Stmt::Block(vec![init, body]);
    }

    Ok(body)
  }

  fn block(&self) -> Result<Vec<Stmt>> {
    let mut statements = vec![];

    while !self.check(&TT::RightBrace) && !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    self.consume(TT::RightBrace, "Expect '}' after block.")?;
    Ok(statements)
  }

  fn expression_statement(&self) -> Result<Stmt> {
    let value = self.expression()?;
    self.consume(TT::Semicolon, "Expect ';' after value.")?;
    Ok(Stmt::Expression(value))
  }

  fn expression(&self) -> Result<Box<Expr>> {
    self.assignment()
  }

  fn assignment(&self) -> Result<Box<Expr>> {
    let expr = self.or()?;

    if self.next_matches(&[TT::Equal]) {
      let equals = self.previous().unwrap();
      let value = self.assignment()?;

      if let Expr::Variable(tok) = *expr {
        Ok(Box::new(Expr::Assign(tok, value)))
      } else {
        Err(Error::Parse(
          equals.clone(),
          format!("invalid assignment target: {}", equals),
        ))
      }
    } else {
      Ok(expr)
    }
  }

  fn or(&self) -> Result<Box<Expr>> {
    let mut expr = self.and()?;

    while self.next_matches(&[TT::Or]) {
      let op = self.previous().unwrap();
      let right = self.and()?;
      expr = Box::new(Expr::Logical(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn and(&self) -> Result<Box<Expr>> {
    let mut expr = self.equality()?;

    while self.next_matches(&[TT::And]) {
      let op = self.previous().unwrap();
      let right = self.equality()?;
      expr = Box::new(Expr::Logical(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn equality(&self) -> Result<Box<Expr>> {
    let mut expr = self.comparison()?;

    while self.next_matches(&[TT::BangEqual, TT::EqualEqual]) {
      let op = self.previous().unwrap();
      let right = self.comparison()?;
      expr = Box::new(Expr::Binary(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn comparison(&self) -> Result<Box<Expr>> {
    let mut expr = self.term()?;

    while self.next_matches(&[
      TT::Greater,
      TT::GreaterEqual,
      TT::Less,
      TT::LessEqual,
    ]) {
      let op = self.previous().unwrap();
      let right = self.term()?;
      expr = Box::new(Expr::Binary(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn term(&self) -> Result<Box<Expr>> {
    let mut expr = self.factor()?;

    while self.next_matches(&[TT::Plus, TT::Minus]) {
      let op = self.previous().unwrap();
      let right = self.factor()?;
      expr = Box::new(Expr::Binary(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn factor(&self) -> Result<Box<Expr>> {
    let mut expr = self.unary()?;

    while self.next_matches(&[TT::Slash, TT::Star]) {
      let op = self.previous().unwrap();
      let right = self.unary()?;
      expr = Box::new(Expr::Binary(expr, op.clone(), right));
    }

    Ok(expr)
  }

  fn unary(&self) -> Result<Box<Expr>> {
    if self.next_matches(&[TT::Bang, TT::Minus]) {
      let op = self.previous().unwrap();
      let right = self.unary()?;
      Ok(Box::new(Expr::Unary(op.clone(), right)))
    } else {
      self.call()
    }
  }

  fn call(&self) -> Result<Box<Expr>> {
    let mut expr = self.primary()?;
    loop {
      if self.next_matches(&[TT::LeftParen]) {
        expr = self.finish_call(expr)?;
      } else {
        break;
      }
    }

    Ok(expr)
  }

  fn finish_call(&self, callee: Box<Expr>) -> Result<Box<Expr>> {
    let mut args = vec![];

    if !self.check(&TT::RightParen) {
      args.push(self.expression()?);

      while self.next_matches(&[TT::Comma]) {
        args.push(self.expression()?);
      }
    }

    if args.len() >= 255 {
      // meh, I'm just gonna skip this here
    }

    let paren = self.consume(TT::RightParen, "Expect ')' after argument list.")?;

    Ok(Box::new(Expr::Call(callee, paren, args)))
  }

  // this sucks
  fn primary(&self) -> Result<Box<Expr>> {
    let next = self.peek().expect("no token to parse in primary()");

    let expr = match next.kind {
      TT::True => Expr::Literal(Literal::Boolean(true)),
      TT::False => Expr::Literal(Literal::Boolean(false)),
      TT::Nil => Expr::Literal(Literal::Nil),
      TT::Number(n) => Expr::Literal(Literal::Number(n)),
      TT::String(ref s) => Expr::Literal(Literal::String(s.clone())),
      TT::LeftParen => {
        self.advance();
        let expr = self.expression()?;
        self.consume(TT::RightParen, "Expect ')' after expression.")?;
        self.rewind(); // silly
        Expr::Grouping(expr)
      },
      TT::Identifier(_) => Expr::Variable(next.clone()),
      _ => {
        return Err(Error::Parse(
          next.clone(),
          format!("expect expression, got {}", next.kind),
        ))
      },
    };

    self.advance();
    Ok(Box::new(expr))
  }

  // helpers
  fn is_at_end(&self) -> bool {
    let next = self.peek();
    next.is_none() || next.unwrap().kind_matches(&TT::EOF)
  }

  fn next_matches(&self, kinds: &[TT]) -> bool {
    for kind in kinds {
      if self.check(kind) {
        self.advance();
        return true;
      }
    }

    false
  }

  fn check(&self, kind: &TT) -> bool {
    if self.is_at_end() {
      false
    } else {
      self.peek().unwrap().kind_matches(kind)
    }
  }

  fn peek(&self) -> Option<&Token> {
    self.tokens.get(*self.current.borrow())
  }

  fn previous(&self) -> Option<&Token> {
    self.tokens.get(*self.current.borrow() - 1)
  }

  fn advance(&self) -> &Token {
    let mut cur = self.current.borrow_mut();
    let token = &self.tokens[*cur];
    *cur += 1;
    token
  }

  fn rewind(&self) {
    let mut cur = self.current.borrow_mut();
    *cur -= 1;
  }

  fn consume(&self, kind: TT, err: &str) -> Result<Token> {
    if self.check(&kind) {
      self.advance();
      Ok(self.previous().unwrap().clone())
    } else {
      Err(Error::Parse(
        self.previous().unwrap().clone(),
        format!("{err}"),
      ))
    }
  }

  fn synchronize(&self) {
    self.advance();
    while !self.is_at_end() {
      if self.previous().unwrap().kind == TT::Semicolon {
        return;
      }

      match self.peek().unwrap().kind {
        TT::Class
        | TT::For
        | TT::Fun
        | TT::If
        | TT::Print
        | TT::Return
        | TT::Var
        | TT::While => return,
        _ => (),
      }

      self.advance();
    }
  }
}
