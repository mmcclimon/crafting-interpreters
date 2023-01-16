use std::cell::RefCell;
use std::collections::HashMap;

use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::{Error, Interpreter, Token};

#[derive(Debug)]
pub struct Resolver<'int> {
  interp: &'int Interpreter,
  errors: RefCell<Vec<Error>>,
  scopes: RefCell<Vec<HashMap<String, bool>>>,
}

impl<'int> Resolver<'int> {
  pub fn new(interp: &'int Interpreter) -> Self {
    Resolver {
      interp,
      errors: RefCell::new(vec![]),
      scopes: RefCell::new(vec![]),
    }
  }

  pub fn resolve(&self, statements: &Vec<Stmt>) {
    for stmt in statements {
      self.resolve_stmt(stmt);
    }
  }

  fn resolve_stmt(&self, statement: &Stmt) {
    match statement {
      Stmt::Block(block) => {
        self.push_scope();
        self.resolve(block);
        self.pop_scope();
      },
      Stmt::Var(name, init) => {
        self.declare(name);
        self.resolve_expr(init);
        self.define(name);
      },
      Stmt::Function(name, params, body) => {
        let name = name.lexeme();
        self.declare(&name);
        self.define(&name);
        self.resolve_function(params, body);
      },
      Stmt::Expression(expr) => {
        self.resolve_expr(expr);
      },
      Stmt::If(cond, then_branch, else_branch) => {
        self.resolve_expr(cond);
        self.resolve_stmt(then_branch);
        self.resolve_stmt(else_branch);
      },
      Stmt::Print(expr) => {
        self.resolve_expr(expr);
      },
      Stmt::Return(_, expr) => {
        self.resolve_expr(expr);
      },
      Stmt::While(cond, body) => {
        self.resolve_expr(cond);
        self.resolve_stmt(body);
      },
      Stmt::Empty => (),
    }
  }

  fn resolve_expr(&self, expr: &Expr) {
    match expr {
      Expr::Variable(token) => {
        if self.scopes.borrow().len() > 0 {
          if let Some(false) =
            self.scopes.borrow().last().unwrap().get(&token.lexeme())
          {
            self.errors.borrow_mut().push(Error::Resolve(
              "Can't read local var in its own initializer".into(),
            ))
          }
        }

        self.resolve_local(expr, &token.lexeme());
      },
      Expr::Assign(token, expr) => {
        self.resolve_expr(expr);
        self.resolve_local(expr, &token.lexeme());
      },
      Expr::Binary(left, _, right) => {
        self.resolve_expr(left);
        self.resolve_expr(right);
      },
      Expr::Logical(left, _, right) => {
        self.resolve_expr(left);
        self.resolve_expr(right);
      },
      Expr::Unary(_, right) => {
        self.resolve_expr(right);
      },
      Expr::Call(callee, _, args) => {
        self.resolve_expr(callee);
        args.iter().for_each(|arg| self.resolve_expr(arg));
      },
      Expr::Grouping(expr) => {
        self.resolve_expr(expr);
      },
      Expr::Literal(_) => (),
    }
  }

  fn resolve_function(&self, params: &Vec<Token>, body: &Vec<Stmt>) {
    self.push_scope();

    for param in params {
      let name = param.lexeme();
      self.declare(&name);
      self.define(&name);
    }

    self.resolve(body);

    self.pop_scope();
  }

  fn push_scope(&self) {
    self.scopes.borrow_mut().push(HashMap::new());
  }

  fn pop_scope(&self) {
    self.scopes.borrow_mut().pop();
  }

  fn declare(&self, name: &str) {
    if self.scopes.borrow().len() == 0 {
      return;
    }

    self
      .scopes
      .borrow_mut()
      .last_mut()
      .unwrap()
      .insert(name.into(), false);
  }

  fn define(&self, name: &str) {
    if self.scopes.borrow().len() == 0 {
      return;
    }

    self
      .scopes
      .borrow_mut()
      .last_mut()
      .unwrap()
      .insert(name.into(), true);
  }

  fn resolve_local(&self, expr: &Expr, name: &str) {
    let scopes = self.scopes.borrow();

    for i in (0..scopes.len()).rev() {
      if scopes[i].contains_key(name) {
        self.interp.resolve(expr, scopes.len() - 1 - i);
        return;
      }
    }
  }
}
