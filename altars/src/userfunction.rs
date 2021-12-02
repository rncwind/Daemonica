use crate::{ast::{Expr, Stmt}, environment::Environment, interpreter::Interpreter, token::Token};


#[derive(PartialEq, Debug, Clone)]
pub struct UserFunction{
    symbol: Token,
    body: Vec<Stmt>,
}

impl UserFunction {
    pub fn new(symbol: Token, body: Vec<Stmt>) -> UserFunction {
        UserFunction{
            symbol,
            body
        }
    }

    pub fn call(&self, interpreter: &mut Interpreter) -> Result<Environment, (Environment, String)> {
        let scope_copy = interpreter.environment.clone();
        match interpreter.interpret_block(self.body.clone(), scope_copy.clone()) {
            Ok(x) => {
                // Execution successful, return the modified scope.
                return Ok(interpreter.environment.clone());
            },
            Err(x) => {
                // Execution went wrong, return error message, and the old scope.
                return Err((scope_copy, x));
            },
        }
    }
}
