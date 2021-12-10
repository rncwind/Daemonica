//! Language-level functions and builtins.
use core::{fmt, time};
use std::{
    collections::HashMap,
    thread,
    time::{SystemTime, UNIX_EPOCH}, io::{self, BufRead}, fs,
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

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Option<Value>, String> {
        let res = (self.func)(interpreter, args)?;
        //interpreter.retval = Some(res);
        Ok(Some(res))
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
            arity: 1,
            func: |_, args| {
                if args.len() != 1 {
                    let emsg = format!("Attempted to call manere with {} args but expected 1", args.len());
                    return Err(emsg);
                }
                let arg = args.first().unwrap();
                match arg {
                    Value::Number(x) => {
                        thread::sleep(time::Duration::from_secs(x.round() as u64));
                        return Ok(Value::Empty);
                    },
                    _ => { return Err(String::from("Attempted to call manere with a non-numeric argument")) }
                }
            },
        })),
    );
    funcs.insert(
        String::from("audire"),
        Some(Value::NativeFn(NativeFn {
            name: "audire".to_string(),
            arity: 0,
            func: |_, _| {
                let mut read = String::new();
                match std::io::stdin().read_line(&mut read) {
                    Ok(_) => {
                        return Ok(Value::String(read.trim_end().to_string()));
                    },
                    Err(e) => {
                        return Err(format!("Encountered error reading line from stdin! {}", e.to_string()));
                    },
                }
            },
        })),
    );
    funcs.insert(
        String::from("legere"),
        Some(Value::NativeFn(NativeFn {
            name: "legere".to_string(),
            arity: 1,
            func: |_, args| {
                if args.len() != 1 {
                    let emsg = format!("Attempted to call legere with {} args but expected 1", args.len());
                    return Err(emsg);
                }
                let arg = args.first().unwrap();
                match arg {
                    Value::String(x) => {
                        let emsg = format!("Unable to read file at {}", x);
                        let read = fs::read_to_string(x).expect(&emsg);
                        return Ok(Value::String(read));
                    }
                    _ => {
                        return Err(String::from("Attempted to call legere with a non-string argument!"));
                    }
                }
            },
        })),
    );
    funcs.insert(
        String::from("mutare"),
        Some(Value::NativeFn(NativeFn {
            name: String::from("mutare"),
            arity: 1,
            func: |_, args| {
                if args.len() != 1 {
                    let emsg = format!("Attempted to call mutare with {} args but only expected 1", args.len());
                    return Err(emsg);
                }
                let arg = args.first().unwrap();
                match arg {
                    Value::String(x) => {
                        let result = x.parse::<f64>();
                        match result {
                            Ok(y) => {
                                return Ok(Value::Number(y));
                            },
                            Err(e) => {
                                let emsg = format!("Error convering {} to a Number. Error was {}", arg, e.to_string());
                                return Err(emsg);
                            },
                        }
                    },
                    Value::Number(x) => {
                        let result = format!("{}", x);
                        return Ok(Value::String(result));
                    },
                    _ => {
                        let emsg = format!("No conversion possible for this type. CAlled mutare with {}", arg);
                        return Err(emsg);
                    }
                }
            },
        }))
    );
    funcs.insert(
        String::from("salvare"),
        Some(Value::NativeFn(NativeFn {
            name: String::from("salvare"),
            arity: 2,
            func: |_, args| {
                dbg!(args.clone());
                if args.len() != 2 {
                    let emsg = format!("Attempted to call mutare with {} args but only expected 1", args.len());
                    return Err(emsg);
                }

                let path = match args.first() {
                    Some(p) => {
                        match p {
                            Value::String(path) => {
                                path
                            },
                            _ => {
                                let emsg = format!("The first argument to salvare should be a string! Got {}", p);
                                return Err(emsg);
                            }
                        }
                    },
                    None => {
                        let emsg = format!("Salvare takes 2 arguments");
                        return Err(emsg);
                    },
                };


                let value = match args.get(1) {
                    Some(p) => {
                        match p {
                            Value::String(path) => {
                                path
                            },
                            _ => {
                                let emsg = format!("The second argument to salvare should be a string! Got {}", p);
                                return Err(emsg);
                            }
                        }
                    },
                    None => {
                        let emsg = format!("Salvare takes 2 arguments");
                        return Err(emsg);
                    },
                };

                match fs::write(path, value) {
                    Ok(_) => {
                        return Ok(Value::Empty);
                    },
                    Err(e) => {
                        let emsg = format!("Encountered error writing file. {}", e.to_string());
                        return Err(emsg);
                    },
                }
            }
        }))
    );
    funcs
}
