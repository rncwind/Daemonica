use crate::{ast::Value, interpreter::Interpreter};

pub trait Callable {
    fn arity(&self, interpeter: &Interpreter) -> usize;
    fn call(&self, interpeter: &mut Interpreter, args: Vec<Value>) -> Result<Value, String>;
}
