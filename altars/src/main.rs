mod ast;
mod astprinter;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod token;
mod tokentype;

use std::fs;
use std::io;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

use rustyline::Editor;

fn main() {
    repl();
}

fn read_file(path: String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn run_file(path: String) {
    let ritual = read_file(path);
    run(ritual.unwrap());
}

fn repl() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("Daemonica> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                run(line.to_string().clone());
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn run(src: String) {
    let mut s: Scanner = Scanner::new(src);
    let tokens = s.scan_tokens();
    let mut p: Parser = Parser::new(tokens);
    let result = p.parse();
    let mut i: Interpreter = Interpreter;
    let result = i.interpret_expr(result);
    println!("{:?}", result);
    //let mut a: AstPrinter = AstPrinter {};
    //a.print(result.clone());
    //println!("{:?}", result.clone());
}
