//! Type of all functions defined in Daemonica.
use core::fmt;
use std::fmt::Display;

use crate::{
    ast::{Expr, Stmt, Value},
    environment::Environment,
    interpreter::Interpreter,
    token::Token,
};

#[derive(PartialEq, Debug, Clone)]
pub struct UserFunction {
    symbol: Token,
    body: Vec<Stmt>,
    paramlist: Vec<Token>,
}

impl UserFunction {
    pub fn new(symbol: Token, body: Vec<Stmt>, paramlist: Vec<Token>) -> UserFunction {
        UserFunction {
            symbol,
            body,
            paramlist,
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<(Environment, Option<Value>), (Environment, String)> {
        //dbg!(args.clone());
        let scope_copy = interpreter.environment.clone();
        let scope_copy = self.parse_arguments(scope_copy.clone(), args.clone());
        match interpreter.interpret_block(self.body.clone(), scope_copy.clone()) {
            Ok(_) => {
                // Execution successful, return the modified scope.
                return Ok((interpreter.environment.clone(), interpreter.retval.clone()));
            }
            Err(x) => {
                // Execution went wrong, return error message, and the old scope.
                return Err((scope_copy, x));
            }
        }
    }

    fn parse_arguments(&self, mut scope: Environment, args: Vec<Value>) -> Environment {
        for (arg, name) in args.iter().zip(self.paramlist.iter()) {
            scope.define(name.lexeme.clone(), Some(arg.clone()));
        }
        scope
    }
}

impl Display for UserFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.symbol.lexeme.clone();
        let params: Vec<String> = self.paramlist.iter().map(|x| x.lexeme.clone()).collect();
        let params = params.join(", ");
        write!(f, "{} :: ({})", name, params)
    }
}
