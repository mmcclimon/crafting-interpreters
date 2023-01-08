use std::fs::File;
use std::io::prelude::*;
use std::process;

use lox::errors::{self, Result};
use lox::scanner::Scanner;

fn main() -> Result<()> {
  let args = std::env::args().skip(1).collect::<Vec<_>>();

  if args.len() > 1 {
    println!("Usage: lox [script]");
    process::exit(64);
  }

  if args.len() == 1 {
    run_file(&args[0])?;
  } else {
    run_prompt()?;
  }

  Ok(())
}

fn run_file(path: &str) -> Result<()> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  run(contents);

  if errors::had_error() {
    process::exit(65);
  }

  Ok(())
}

fn run_prompt() -> Result<()> {
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();

  loop {
    print!("> ");
    stdout.flush().unwrap();

    let mut line = String::new();
    let bytes_read = stdin.read_line(&mut line)?;

    if bytes_read == 0 {
      break;
    }

    run(line);
    errors::clear_error();
  }

  Ok(())
}

// returns hadError, effectively
fn run(source: String) {
  let mut scanner = Scanner::new(source);

  for token in scanner.scan_tokens() {
    println!("{token}");
  }
}
