mod ast;

mod error;
use error::LoxError;

mod parser;
use parser::Parser;

mod scanner;
use crate::scanner::Scanner;

mod tokens;
use tokens::{Span, Token, TokenType};

mod interpreter;
use interpreter::Interpreter;

mod environment;

use std::env;
use std::io::{self, stdin, stdout, Write};

fn run_file(path: &str) -> io::Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    execute(&file_content);
    Ok(())
}

// the result is useless for now but will be useful eventually
fn execute(source: &str) -> Result<(), LoxError> {
    let scanner = Scanner::new(source).filter(|x| x.token_type != TokenType::Whitespace);
    //TODO: use the iterator instead of collecting
    let mut tokens: Vec<Token> = scanner.collect();

    for token in &tokens {
        println!("{token:?}");
    }

    let mut parser = Parser::new(source, tokens);
    let ast = &parser.parse();

    match ast {
        Ok(x) => {
            println!("{:#?}", x)
        }
        Err(e) => {
            e.report();
            std::process::exit(65);
        }
    }

    let mut interpreter = Interpreter::new(source);
    let expr = interpreter.interpret(ast.as_ref().unwrap());

    match expr {
        Ok(ref x) => { /*println!("{:#?}", x)*/ }
        Err(ref e) => {
            e.report();
            std::process::exit(70);
        }
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

        execute(&line_input);
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let args_length = cli_args.len() - 1;

    match args_length {
        n if n > 2 => println!("Usage: rlox [script] optionally: --verbose"),
        1 => run_file(&cli_args[1]).expect("Failed to run file"),
        _ => run_prompt(),
    }
}
