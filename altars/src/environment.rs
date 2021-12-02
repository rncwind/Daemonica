use std::collections::HashMap;

use crate::{ast::Value, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
    //pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            //enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from(e: Environment) -> Environment {
        Environment {
            values: e.values.clone(),
        }
    }

    pub fn from_ht(other: HashMap<String, Option<Value>>) -> Environment {
        Environment {
            values: other.clone(),
        }
    }

    pub fn define(&mut self, name: String, val: Option<Value>) {
        self.values.insert(name, val);
    }

    pub fn get(&mut self, name: Token) -> Option<Value> {
        let symbol = name.lexeme.clone();
        match self.values.get(&symbol) {
            Some(val) => {
                return val.clone();
            },
            None => {
                return None;
            },
        }
    }

    pub fn assign(&mut self, name: Token, val: &Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, Some(val.clone()));
            return Ok(());
        } else {
            let emsg = format!("Error: Tried to assign value {} to undefined variable {}", name.lexeme, val.clone());
            return Err(emsg);
        }
    }

    pub fn merge_defs(&self, other: HashMap<String, Option<Value>>) -> Environment {
        let mut new = self.clone();
        new.values.extend(other);
        new
    }

    pub fn merge_envs(&self, other: Environment) -> Environment {
        let other = other.values.clone();
        self.merge_defs(other)
    }

    // pub fn get(&mut self, name: Token) -> Option<Value> {
    //     let symbol = name.lexeme.clone();
    //     if self.values.contains_key(&name.lexeme) {
    //         return self.values.get(&symbol).unwrap().clone();
    //     } else {
    //         dbg!(self.clone());
    //         // Check any enclosing scopes that we have.
    //         match self.enclosing {
    //             Some(_) => {
    //                 return self
    //                     .enclosing
    //                     .as_ref()
    //                     .unwrap()
    //                     .values
    //                     .get(&symbol)
    //                     .unwrap()
    //                     .clone();
    //             }
    //             _ => None,
    //         }
    //     }
    // }

    // pub fn assign(&mut self, name: Token, val: &Value) -> Result<(), String> {
    //     if self.values.contains_key(&name.lexeme) {
    //         self.define(name.lexeme, Some(val.clone()));
    //         return Ok(());
    //     }

    //     match &mut self.enclosing {
    //         Some(enc) => {
    //             enc.assign(name.clone(), val)?;
    //             return Ok(());
    //         }
    //         _ => {
    //             let emsg = format!("Undefined variable {}", name.lexeme);
    //             Err(emsg)
    //         }
    //     }
    // }

    // pub fn merge(&mut self, other: HashMap<String, Option<Value>>) {
    //     self.values.extend(other);
    // }
}
