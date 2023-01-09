use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{Token, TokenType};

// These are the two functions defined in the book. I'm not sure I'll keep using
// them, but let's do, for the time being.
pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, msg: &str) {
  report(line, "", msg);
}

fn report(line: usize, loc: &str, msg: &str) {
  eprintln!("[line {line}] Error{loc}: {msg}");
  HAD_ERROR.swap(true, Ordering::Relaxed);
}

// keep the atomic nonsense in here
pub fn had_error() -> bool {
  HAD_ERROR.load(Ordering::Relaxed)
}

pub fn clear_error() {
  HAD_ERROR.swap(false, Ordering::Relaxed);
}

// More idiomatic rust error handling below.

#[derive(Debug)]
pub enum Error {
  Io(IoError),
  Scan(usize, String),
  Parse(Token, String),
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
      Error::Parse(token, msg) => {
        if token.kind == TokenType::EOF {
          write!(f, "[line {}] Error at end: {msg}", token.line)
        } else {
          write!(
            f,
            "[line {}] Error at {}: {msg}",
            token.line,
            token.lexeme()
          )
        }
      },
    }
  }
}
