mod scanner;
mod tokenizer;

use crate::scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn throw_error(line: usize, message: &str) {
    todo!("Implement had_error");
    report_error(line, message);
}

fn report_error(line: usize, message: &str) {
    panic!("Error on line {}:\n{}", line, message);
}

fn read_file(path: &str, errored: bool) {
    let path = Path::new(&path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("couldn't read {display}: {why}"),
        Ok(_) => (),
    }

    if let Err(why) = file.read_to_string(&mut file_content) {
        panic!("couldn't read {display}: {why}")
    }

    if errored {
        panic!();
    }

    execute(file_content);
}

fn execute(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_prompt() {
    loop {
        let mut line_input: String = String::new();
        print!("> ");
        stdout().flush().expect("Failed to flush");
        stdin()
            .read_line(&mut line_input)
            .expect("Failed to read line");

        execute(line_input);
    }
}

fn main() {
    println!("Rlox");

    let cli_args: Vec<String> = env::args().collect();

    let args_length = cli_args.len() - 1;

    let mut errored = false;

    match args_length {
        n if n > 1 => println!("Usage: rlox [script]"),
        1 => read_file(&cli_args[1], errored),
        _ => run_prompt(),
    }
}
