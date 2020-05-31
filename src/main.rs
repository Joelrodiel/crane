use std::env;
use std::fs;
use std::io::{self, stdin, Write};

mod tokens;
mod scanner;

use tokens::Token;
use scanner::scan_tokens;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: crane [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let data = fs::read_to_string(path);

    let _ = match data {
        Ok(content) => run(&content),
        Err(err) => {
            panic!("Error reading file: {:?}", err)
        },
    };
}

fn run_prompt() {
    loop {
        print!("crane> ");
        if let Err(err) = io::stdout().flush() {
            println!("Could not parse input: {:?}", err);
        }
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        run(&line);
    }
}

fn run(data: &str) {
    let tokens: Vec<Token> = scan_tokens(data);
    println!("Data: {}", data);
    for v in tokens {
        v.print();
    }
}
