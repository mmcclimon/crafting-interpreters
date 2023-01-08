use crate::token::Token;

pub struct Scanner {}

impl Scanner {
  pub fn new(_source: String) -> Self {
    Scanner {}
  }

  // wrong
  pub fn scan_tokens(&self) -> Vec<Token> {
    vec![]
  }
}
