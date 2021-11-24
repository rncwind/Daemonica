use std::collections::HashMap;

use crate::{ast::Value, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // Create a new environment, that itself contains an enclosing
    // environment. Needed for scoping!
    pub fn new_with_enclosing(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, val: Option<Value>) {
        self.values.insert(name, val);
    }

    pub fn get(&mut self, name: Token) -> Option<Value> {
        let symbol = name.lexeme.clone();
        if self.values.contains_key(&name.lexeme) {
            return self.values.get(&symbol).unwrap().clone();
        } else {
            dbg!(self.clone());
            // Check any enclosing scopes that we have.
            match self.enclosing {
                Some(_) => {
                    return self
                        .enclosing
                        .as_ref()
                        .unwrap()
                        .values
                        .get(&symbol)
                        .unwrap()
                        .clone();
                }
                _ => None,
            }
        }
    }

    pub fn assign(&mut self, name: Token, val: &Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, Some(val.clone()));
            return Ok(());
        }

        match &mut self.enclosing {
            Some(enc) => {
                enc.assign(name.clone(), val)?;
                return Ok(());
            }
            _ => {
                let emsg = format!("Undefined variable {}", name.lexeme);
                Err(emsg)
            }
        }
    }

    pub fn merge(&mut self, other: HashMap<String, Option<Value>>) {
        self.values.extend(other);
    }
}
