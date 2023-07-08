use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn read_file(path: String) {
    let path = Path::new(&path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    execute(file_content);
}

fn execute(code: String) -> String {
    println!("{}", code);
    code
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
    if args_length > 1 {
        println!("Usage: rlox [script]");
    } else if args_length == 1 {
        read_file(cli_args[1].clone());
    } else {
        run_prompt();
    }
}
