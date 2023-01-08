#[derive(Debug)]
pub enum TokenType {
  // single-char tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // one/two-char tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // literals
  Identifier(String),
  String(String),
  Number(f64),

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
  line: usize,
}

impl Token {
  pub fn new(kind: TokenType, lexeme: String, line: usize) -> Self {
    Token { kind, lexeme, line }
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{:?}", self.kind)
  }
}

impl TokenType {
  // returns an identifier or reserved word
  pub fn new_identifier(s: String) -> Self {
    match s.as_str() {
      "and" => Self::And,
      "class" => Self::Class,
      "else" => Self::Else,
      "false" => Self::False,
      "fun" => Self::Fun,
      "for" => Self::For,
      "if" => Self::If,
      "nil" => Self::Nil,
      "or" => Self::Or,
      "print" => Self::Print,
      "return" => Self::Return,
      "super" => Self::Super,
      "this" => Self::This,
      "true" => Self::True,
      "var" => Self::Var,
      "while" => Self::While,
      _ => Self::Identifier(s),
    }
  }
}
