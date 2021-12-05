//! Language-level functions and builtins.
use core::{fmt, time};
use std::{
    collections::HashMap,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use fxhash::hash64;

use crate::{ast::Value, callable::Callable, interpreter::Interpreter};

#[derive(Clone)]
pub struct NativeFn {
    name: String,
    arity: usize,
    // We can implement a callable as a closure type here.
    func: fn(&mut Interpreter, Vec<Value>) -> Result<Value, String>,
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        //self.name == other.name && self.arity == other.arity
        let selfhash = hash64(&self.stringify_for_hash());
        let otherhash = hash64(&other.stringify_for_hash());
        selfhash == otherhash
    }
}

impl NativeFn {
    pub fn stringify_for_hash(&self) -> String {
        format!("{}.{}", self.name, self.arity)
    }

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, String> {
        let res = (self.func)(interpreter, args)?;
        interpreter
            .environment
            .define(String::from("tempus"), Some(res.clone()));
        Ok(res)
    }
}

// impl Callable for NativeFn {
//     fn arity(&self, interpreter: &Interpreter) -> usize {
//         self.arity
//     }

// }

impl fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeFn({}.{})", self.name, self.arity)
    }
}

impl fmt::Display for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeFn({})", self.name)
    }
}

pub fn generate_native_functions() -> HashMap<String, Option<Value>> {
    let mut funcs: HashMap<String, Option<Value>> = HashMap::new();
    funcs.insert(
        String::from("horologium"),
        Some(Value::NativeFn(NativeFn {
            name: "horologium".to_string(),
            arity: 0,
            func: |_, _| {
                let start_time = SystemTime::now();
                Ok(Value::Number(
                    start_time.duration_since(UNIX_EPOCH).unwrap().as_millis() as f64,
                ))
            },
        })),
    );
    funcs.insert(
        String::from("manere"),
        Some(Value::NativeFn(NativeFn {
            name: "manere".to_string(),
            arity: 0,
            func: |_, _| {
                thread::sleep(time::Duration::from_secs(5));
                Ok(Value::Empty)
            },
        })),
    );
    funcs
}
