mod ast;
mod environment;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod token;
mod tokentype;
mod callable;
mod nativefn;
mod userfunction;

use std::fs;
use std::io;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

use rustyline::Editor;

use clap::{AppSettings, Parser as ClapParser};

#[derive(ClapParser)]
#[clap(version = "0.1")]
struct Opts {
    sourcefile: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.sourcefile {
        Some(x) => {
            run_file(x)
        },
        _ => {
            repl();
        }
    }
}

fn read_file(path: String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn run_file(path: String) {
    let mut interpreter = Interpreter::new();
    let ritual = read_file(path);
    run(ritual.unwrap(), &mut interpreter);
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();
    loop {
        let readline = rl.readline("Daemonica> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                run(line.to_string().clone(), &mut interpreter);
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn run(src: String, i: &mut Interpreter) {
    let mut s: Scanner = Scanner::new(src);
    let tokens = s.scan_tokens();
    let mut p: Parser = Parser::new(tokens);
    let parsed = p.parse();
    //println!("{:#?}", parsed);
    let result = i.interpret(parsed);
    //println!("{:#?}", result);
}
