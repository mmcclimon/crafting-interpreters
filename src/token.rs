type TT = TokenType;

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenType,
  pub line: usize,
}

impl Token {
  pub fn new(kind: TokenType, line: usize) -> Self {
    Token { kind, line }
  }

  pub fn lexeme(&self) -> String {
    match &self.kind {
      TT::Number(n) => n.to_string(),
      TT::String(s) => format!("\"{s}\""),
      TT::Identifier(s) => s.clone(),
      _ => self.kind.as_str().to_string(),
    }
  }

  pub fn kind_matches(&self, other: &TokenType) -> bool {
    use std::mem::discriminant;
    discriminant(&self.kind) == discriminant(other)
  }

  pub fn is_identifier(&self) -> bool {
    match self.kind {
      TT::Identifier(_) => true,
      _ => false,
    }
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

  fn as_str(&self) -> &str {
    match self {
      TT::LeftParen => "(",
      TT::RightParen => ")",
      TT::LeftBrace => "{",
      TT::RightBrace => "}",
      TT::Comma => ",",
      TT::Dot => ".",
      TT::Minus => "-",
      TT::Plus => "+",
      TT::Semicolon => ";",
      TT::Slash => "/",
      TT::Star => "*",
      TT::Bang => "!",
      TT::BangEqual => "!=",
      TT::Equal => "=",
      TT::EqualEqual => "==",
      TT::Greater => ">",
      TT::GreaterEqual => ">=",
      TT::Less => "<",
      TT::LessEqual => "<=>",
      TT::Identifier(s) => &s,
      TT::String(s) => &s,
      TT::Number(_) => "__SOME NUMBER__", // lol what
      TT::And => "and",
      TT::Class => "class",
      TT::Else => "else",
      TT::False => "false",
      TT::Fun => "fun",
      TT::For => "for",
      TT::If => "if",
      TT::Nil => "nil",
      TT::Or => "or",
      TT::Print => "print",
      TT::Return => "return",
      TT::Super => "super",
      TT::This => "this",
      TT::True => "true",
      TT::Var => "var",
      TT::While => "while",
      TT::EOF => "eof",
    }
  }
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.as_str())
  }
}
