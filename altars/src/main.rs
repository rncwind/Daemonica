mod tokentype;
mod token;
mod literals;
mod scanner;
mod ast;
mod interpreter;
mod astprinter;

use std::env;
use std::fs;
use std::io;

use ast::Expr;
use literals::Literal;
use tokentype::TokenType;
use token::Token;

use crate::astprinter::AstPrinter;

fn main() {
    let expression: Expr = Expr::Binary(
        Box::new(Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), Literal::Empty, 1),
            Box::new(Expr::Literal(Literal::Number(123.0))),
        )),
        Token::new(TokenType::Star, "*".to_string(), Literal::Empty, 1),
        Box::new(Expr::Grouping(
            Box::new(Expr::Literal(Literal::Number(45.67)))
        )),
    );
    let mut p = AstPrinter{};
    println!("{}", p.print(expression));
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
