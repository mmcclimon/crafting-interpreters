use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
  source: Vec<char>,
  tokens: Vec<Token>,
  current: usize,
  start: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: String) -> Self {
    Scanner {
      source: source.chars().collect(),
      tokens: vec![],
      current: 0,
      start: 0,
      line: 1,
    }
  }

  // really this should return an iterator or something, but hey
  pub fn scan_tokens(&mut self) -> &Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self
      .tokens
      .push(Token::new(TokenType::EOF, "".into(), self.line));

    &self.tokens
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn scan_token(&mut self) {
    let c = self.advance();

    use TokenType as TT;

    match c {
      // single chars
      '(' => self.add_token(TT::LeftParen),
      ')' => self.add_token(TT::RightParen),
      '{' => self.add_token(TT::LeftBrace),
      '}' => self.add_token(TT::RightBrace),
      ',' => self.add_token(TT::Comma),
      '.' => self.add_token(TT::Dot),
      '-' => self.add_token(TT::Minus),
      '+' => self.add_token(TT::Plus),
      ';' => self.add_token(TT::Semicolon),
      '*' => self.add_token(TT::Star),

      // double chars
      '!' => {
        if self.next_matches('=') {
          self.add_token(TT::BangEqual)
        } else {
          self.add_token(TT::Bang)
        }
      },

      '=' => {
        if self.next_matches('=') {
          self.add_token(TT::EqualEqual)
        } else {
          self.add_token(TT::Equal)
        }
      },

      '<' => {
        if self.next_matches('=') {
          self.add_token(TT::LessEqual)
        } else {
          self.add_token(TT::Less)
        }
      },

      '>' => {
        if self.next_matches('=') {
          self.add_token(TT::GreaterEqual)
        } else {
          self.add_token(TT::Greater)
        }
      },

      // comments or slash
      '/' => {
        if self.next_matches('/') {
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
          // a comment!
        } else {
          self.add_token(TT::Slash)
        }
      },

      // whitespace
      ' ' | '\r' | '\t' => (),
      '\n' => {
        self.line += 1;
      },

      // string and numeric literals
      '"' => self.read_string(),
      c if c.is_ascii_digit() => self.read_number(),

      // identifiers and keywords
      c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(),

      _ => panic!("ohno"),
    }
  }

  fn advance(&mut self) -> char {
    let c = self.source[self.current];
    self.current += 1;
    c
  }

  fn peek(&mut self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current]
    }
  }

  fn peek_next(&mut self) -> char {
    if self.current + 1 > self.source.len() {
      '\0'
    } else {
      self.source[self.current + 1]
    }
  }

  fn current_string(&self) -> String {
    self.source[self.start..self.current].iter().collect()
  }

  fn next_matches(&mut self, expect: char) -> bool {
    if self.is_at_end() || self.source[self.current] != expect {
      return false;
    }

    self.current += 1;
    true
  }

  fn add_token(&mut self, kind: TokenType) {
    self
      .tokens
      .push(Token::new(kind, self.current_string(), self.line));
  }

  fn read_string(&mut self) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line += 1;
      }

      self.advance();
    }

    if self.is_at_end() {
      panic!("unterminated string");
    }

    self.advance(); // closing quote
    let val: String = self.source[self.start + 1..self.current - 1]
      .iter()
      .collect();

    self.add_token(TokenType::String(val));
  }

  fn read_number(&mut self) {
    while self.peek().is_ascii_digit() {
      self.advance();
    }

    if self.peek() == '.' && self.peek_next().is_ascii_digit() {
      self.advance(); // eat the dot
                      //
      while self.peek().is_ascii_digit() {
        self.advance();
      }
    }

    let val: f64 = self.current_string().parse().expect("bogus numeric value");
    self.add_token(TokenType::Number(val));
  }

  fn read_identifier(&mut self) {
    let char_ok =
      |c: char| c == '_' || c.is_ascii_alphabetic() || c.is_ascii_digit();

    while char_ok(self.peek()) {
      self.advance();
    }

    self.add_token(TokenType::new_identifier(self.current_string()));
  }
}
