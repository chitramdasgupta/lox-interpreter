mod lexical_analysis;

use lexical_analysis::lexer::Lexer;
use std::io::{stdin, BufRead};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: lox_interpreter [script]"),
    }
}

fn run_file(path: &str) {
    let read = fs::read(path).expect("Could not read file!");
    let result = String::from_utf8(read).expect("Please enter a UTF-8 file!");
    run(&result);
}

fn run_repl() {
    println!(">");
    let handle = stdin().lock();

    for line in handle.lines() {
        run(&line.expect("Could not read line!"));
    }
}

fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    let (tokens, lexical_errors) = lexer.scan_tokens();

    for error in lexical_errors {
        eprintln!("{}", error);
    }

    for token in tokens {
        println!("{}", token);
    }
}
