use std::cell::RefCell;

use crate::errors::{Error, Result};
use crate::expr::{Expr, Literal};
use crate::token::{Token, TokenType as TT};

#[derive(Debug)]
pub struct Parser {
  tokens: Vec<Token>,
  current: RefCell<usize>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      current: RefCell::new(0),
    }
  }

  pub fn parse(&self) -> Result<Box<Expr>> {
    self.expression()
  }

  fn expression(&self) -> Result<Box<Expr>> {
    self.equality()
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
      self.primary()
    }
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
        self.consume(TT::RightParen, "Expect ') after expression.")?;
        self.rewind(); // silly
        Expr::Grouping(expr)
      },
      _ => return Err(Error::Parse(next.clone(), "expect expression".into())),
    };

    self.advance();
    Ok(Box::new(expr))
  }

  // helpers
  fn is_at_end(&self) -> bool {
    *self.current.borrow() >= self.tokens.len()
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

  fn consume(&self, kind: TT, err: &str) -> Result<()> {
    if self.check(&kind) {
      self.advance();
      Ok(())
    } else {
      Err(Error::Parse(
        self.previous().unwrap().clone(),
        format!("{err}"),
      ))
    }
  }

  #[allow(unused)]
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
