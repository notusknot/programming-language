mod error;
mod expr;
mod scanner;
mod tokenizer;

use error::LoxError;

use crate::scanner::Scanner;
use std::env;
use std::io::{self, stdin, stdout, Write};

fn run_file(path: &str) -> io::Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    if execute(file_content).is_err() {
        // Ignore: error was already reported
        std::process::exit(65);
    }

    Ok(())
}

fn execute(source: String) -> Result<(), LoxError> {
    let scanner = Scanner::new(source);

    for token in scanner {
        println!("{token:?}");
    }

    Ok(())
}

fn run_prompt() {
    println!("Rlox");
    loop {
        let mut line_input: String = String::new();
        print!("> ");
        stdout().flush().expect("Failed to flush");
        stdin()
            .read_line(&mut line_input)
            .expect("Failed to read line");

        if execute(line_input).is_err() {
            eprintln!("Failed to execute file");
        };
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let args_length = cli_args.len() - 1;

    match args_length {
        n if n > 1 => println!("Usage: rlox [script]"),
        1 => run_file(&cli_args[1]).expect("Failed to run file"),
        _ => run_prompt(),
    }
}
