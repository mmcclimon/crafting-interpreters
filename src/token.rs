#[allow(unused)]
#[derive(Debug)]
enum TokenType {
  // single-char tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Commma,
  Dot,
  Minu,
  Plus,
  Semicolon,
  Slash,
  Star,

  // one/two-char tokens,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // literals
  Identifier,
  LString, // to avoid name clash
  Number,

  // keywords
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  EOF,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Token {
  kind: TokenType,
  lexeme: String,
  // literal:
  line: usize,
}

impl Token {}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{:?} {}", self.kind, self.lexeme)
  }
}
