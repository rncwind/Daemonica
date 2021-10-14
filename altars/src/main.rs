mod tokentype;
mod token;
mod literals;
mod scanner;

use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => match run_file(args.get(1).unwrap().clone()) {
            _ => {}
        },
        _ => {
            println!("Usage: altars [ritual]");
        }
    }
}

fn read_file(path: String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn run_file(path: String) -> Result<(), ()> {
    let ritual = read_file(path);
    Ok(())
}

fn repl() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                run(input.clone());
            }
            Err(_x) => {
                break;
            }
        }
    }
}

fn run(src: String) {
    todo!("Scan and tokenise");
}
