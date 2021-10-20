mod ast;
mod astprinter;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod token;
mod tokentype;

use std::env;
use std::fs;
use std::io;

use ast::Expr;
use literals::Literal;
use parser::Parser;
use scanner::Scanner;
use token::Token;
use tokentype::TokenType;

use crate::astprinter::AstPrinter;

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
    let mut s: Scanner = Scanner::new(src);
    let tokens = s.scan_tokens();
    let mut p: Parser = Parser::new(tokens);
    let result = p.parse();
    let mut a: AstPrinter = AstPrinter{};
    a.print(result);
    //println!("{:?}", result);
}
