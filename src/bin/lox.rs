use std::fs::File;
use std::io::prelude::*;
use std::process;

use lox::errors;
use lox::{Interpreter, Parser, Result, Scanner};

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

  let res = run(contents);

  // this smells
  let mut errored = errors::had_error();

  if let Err(e) = res {
    eprintln!("{e}");
    errored = true;
  }

  if errored {
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

    let res = run(line);

    if let Err(e) = res {
      eprintln!("{e}")
    }

    errors::clear_error();
  }

  Ok(())
}

// returns hadError, effectively
fn run(source: String) -> Result<()> {
  let scanner = Scanner::new(source);
  let parser = Parser::new(scanner.into_tokens()?);
  let interpreter = Interpreter {};

  let statements = parser.parse()?;

  interpreter.interpret(statements)?;

  Ok(())
}
