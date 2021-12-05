//! State, Binding, Scope and name tracking.
use std::collections::HashMap;

use crate::{ast::Value, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn from(e: Environment) -> Environment {
        Environment {
            parent: None,
            values: e.values.clone(),
        }
    }

    pub fn from_ht(other: HashMap<String, Option<Value>>) -> Environment {
        Environment {
            parent: None,
            values: other.clone(),
        }
    }

    pub fn with_parent(parent: Environment) -> Environment {
        Environment {
            parent: Some(Box::new(parent)),
            values: HashMap::new(),
        }
    }

    pub fn from_with_parent(e: Environment, parent: Environment) -> Environment {
        Environment {
            parent: Some(Box::new(parent)),
            values: e.values.clone(),
        }
    }

    pub fn from_ht_with_parent(other: HashMap<String, Option<Value>>, parent: Environment) -> Environment {
        Environment {
            parent: Some(Box::new(parent)),
            values: other.clone(),
        }
    }

    pub fn define(&mut self, name: String, val: Option<Value>) {
        self.values.insert(name, val);
    }

    pub fn get(&self, name: Token) -> Option<Value> {
        let symbol = name.lexeme.clone();
        match self.values.get(&symbol) {
            Some(val) => {
                return val.clone();
            }
            None => {
                return None;
            }
        }
    }

    pub fn assign(&mut self, name: Token, val: &Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, Some(val.clone()));
            return Ok(());
        } else {
            let emsg = format!(
                "Error: Tried to assign value {} to undefined variable {}",
                name.lexeme,
                val.clone()
            );
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
}
