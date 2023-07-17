mod error;
mod expr;
mod parser;
use parser::*;
mod scanner;
mod tokenizer;
use tokenizer::{Span, Token, TokenType::*};

mod ast_printer;
use ast_printer::*;

use error::LoxError;

use crate::scanner::Scanner;
use std::env;
use std::io::{self, stdin, stdout, Write};

fn run_file(path: &str) -> io::Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    if execute(&file_content).is_err() {
        // Ignore: error was already reported
        std::process::exit(65);
    }

    Ok(())
}

// the result is useless for now but will be useful eventually
fn execute(source: &str) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);

    //temporary
    //TODO: use the iterator instead of collecting
    let mut tokens: Vec<Token> = scanner.collect();
    /*
    let last_span = tokens.last().unwrap().span;
    tokens.push(Token {
        token_type: Eof,
        span: Span::from(last_span.end + 1..last_span.end + 1),
    });
    */

    println!("Tokens:");
    for token in &tokens {
        println!("{token:?}");
    }

    println!("\nExpressions:");

    let mut parser = Parser::new(source, tokens);

    let printer = AstPrinter { source };
    println!("AST Printer:\n{}", printer.print(&parser.parse()?)?);

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

        if execute(&line_input).is_err() {
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
