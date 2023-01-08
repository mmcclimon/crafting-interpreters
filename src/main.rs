pub mod scanner;
pub mod token;

use std::fs::File;
use std::io::prelude::*;
use std::process;

use crate::scanner::Scanner;

fn main() -> std::io::Result<()> {
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

fn run_file(path: &str) -> std::io::Result<()> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(())
}

fn run_prompt() -> std::io::Result<()> {
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();

  loop {
    print!("> ");
    stdout.flush().unwrap();

    let mut line = String::new();
    let read = stdin.read_line(&mut line)?;

    if read == 0 {
      break;
    }

    let err = run(line);

    if err {
      process::exit(65);
    }
  }

  Ok(())
}

// returns hadError, effectively
fn run(source: String) -> bool {
  let scanner = Scanner::new(source);

  for token in scanner.scan_tokens() {
    println!("{token}");
  }

  false
}

#[allow(unused)]
fn error(line: usize, msg: &str) {
  report(line, "", msg);
}

#[allow(unused)]
fn report(line: usize, loc: &str, msg: &str) {
  eprint!("[ line {line}] Error {loc}: {msg}");
}
