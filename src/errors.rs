use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

use crate::{Token, TokenType};

// I have replaced the error handling in the book with more idiomatic rust.

#[derive(Debug)]
pub enum Error {
  Io(IoError),
  Scan(usize, String),
  Parse(Token, String),
  ParseFailed,
  Runtime(Token, String),
  TryFrom(String),
}

impl Error {
  fn line_display(&self) -> String {
    match self {
      Error::Parse(token, msg) | Error::Runtime(token, msg) => {
        if token.kind == TokenType::EOF {
          format!("[line {}] Error at end: {msg}", token.line)
        } else {
          format!("[line {}] Error at {}: {msg}", token.line, token.lexeme())
        }
      },
      _ => unimplemented!(),
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;

impl StdError for Error {}

impl From<IoError> for Error {
  fn from(err: IoError) -> Self {
    Error::Io(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Io(err) => write!(f, "{}", err),
      Error::Scan(line, msg) => write!(f, "[line {line}] Scan error: {msg}"),
      Error::Parse(_, _) => write!(f, "{}", self.line_display()),
      Error::ParseFailed => write!(f, "parse failed"),
      Error::Runtime(_, _) => write!(f, "{}", self.line_display()),
      Error::TryFrom(err) => write!(f, "{}", err),
    }
  }
}
