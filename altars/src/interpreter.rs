//! A Tree Walk Interpreter for the Daemonica Language
use crate::ast::ASTNode;
use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Value;
use crate::environment::Environment;
use crate::literals::Literal;
use crate::nativefn;
use crate::token::Token;
use crate::tokentype::TokenType;
use crate::userfunction::UserFunction;

#[derive(Debug)]
pub struct Interpreter {
    pub environment: Environment,
    pub retval: Option<Value>,
}

//impl<T> Visitor<T> for Interpreter {
impl Interpreter {
    pub fn new() -> Interpreter {
        let environment = Environment::from_ht(nativefn::generate_native_functions());
        return Interpreter {
            environment,
            retval: None,
        };
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) -> Result<Vec<Value>, String> {
        let mut results: Vec<Value> = Vec::new();
        for node in nodes {
            match node {
                ASTNode::StmtNode(x) => match self.interpret_stmt(x) {
                    Ok(y) => {
                        results.push(y);
                    }
                    Err(y) => {
                        println!("Encountered an error {}", y);
                        println!("Environment at this state was {:#?}", self.environment);
                        return Err(y);
                    }
                },
                ASTNode::ExprNode(x) => match self.interpret_expr(x) {
                    Ok(y) => {
                        results.push(y);
                    }
                    Err(y) => {
                        println!("Encountered an error {}", y);
                        println!("Environment at this state was {:#?}", self.environment);
                        return Err(y);
                    }
                },
            }
        }

        return Ok(results);
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Assign(name, value) => {
                return self.interpret_assignment(name, *value);
            }
            Expr::Binary(left, oper, right) => {
                return self.interpret_binary(*left, oper, *right);
            }
            Expr::Call(callee, paren, args) => self.interpret_call(*callee, paren, args),
            Expr::Grouping(expression) => {
                return self.interpret_expr(*expression);
            }
            Expr::Literal(value) => {
                return self.interpret_literal(value);
            }
            Expr::Logic(left, operator, right) => {
                return self.interpret_logical(*left, operator, *right);
            }
            Expr::Unary(operator, right) => {
                return self.interpret_unary(operator, *right);
            }
            Expr::Variable(name) => self.interpret_var_expr(name),
        }
    }

    pub fn interpret_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Block(stmts) => self.interpret_block(stmts, self.environment.clone()),
            Stmt::Expression(expr) => self.interpret_expr(expr),
            Stmt::Function(name, params, body) => self.interpret_function(name, body, params),
            Stmt::If(cond, thenb, elseb) => self.interpret_if(cond, thenb, elseb),
            Stmt::Return(tok, val) => self.interpret_return(tok, val),
            Stmt::Var(tok, initializer) => self.interpret_var_stmt(tok, initializer),
            Stmt::While(cond, body) => self.interpret_while(&cond, body),
            Stmt::Print(expr) => self.interpret_print(expr),
        }
    }

    fn interpret_function(
        &mut self,
        name: Token,
        body: Vec<Stmt>,
        params: Vec<Token>,
    ) -> Result<Value, String> {
        let fun = Value::UserFn(UserFunction::new(name.clone(), body, params));
        self.environment.define(name.lexeme, Some(fun.clone()));
        Ok(Value::Empty)
    }

    fn interpret_return(&mut self, tok: Token, val: Option<Expr>) -> Result<Value, String> {
        match val {
            Some(x) => {
                let evaled = self.interpret_expr(x)?;
                let val: Value = evaled.into();
                self.retval = Some(val.clone());
                return Ok(val.clone());
            }
            None => {
                self.retval = None;
                return Ok(Value::Empty);
            }
        }
    }

    fn interpret_print(&mut self, expr: Expr) -> Result<Value, String> {
        let val = self.interpret_expr(expr)?;
        println!("{}", val);
        return Ok(Value::Empty);
    }

    pub fn interpret_block(&mut self, stmts: Vec<Stmt>, env: Environment) -> Result<Value, String> {
        let prevenv = self.environment.clone();
        self.environment = env;
        for stmt in stmts {
            match self.interpret_stmt(stmt) {
                Ok(_) => {}
                Err(x) => {
                    self.environment = prevenv;
                    return Err(x);
                }
            }
        }
        Ok(Value::Empty)
    }

    fn interpret_var_stmt(
        &mut self,
        tok: Token,
        initializer: Option<Expr>,
    ) -> Result<Value, String> {
        let value = match initializer {
            Some(x) => Some(self.interpret_expr(x).unwrap()),
            None => None,
        };
        self.environment.define(tok.lexeme, value);
        return Ok(Value::Empty);
    }

    fn interpret_while(&mut self, cond: &Expr, body: Box<Stmt>) -> Result<Value, String> {
        while Interpreter::is_truthy(self.interpret_expr(cond.clone())?) {
            self.interpret_stmt(*body.clone())?;
        }
        Ok(Value::Empty)
    }

    fn interpret_if(
        &mut self,
        cond: Expr,
        thenb: Box<Stmt>,
        elseb: Box<Option<Stmt>>,
    ) -> Result<Value, String> {
        // If our condition is truthy, evaluate the then branch
        if Interpreter::is_truthy(self.interpret_expr(cond)?) {
            return self.interpret_stmt(*thenb);
        } else {
            // If our condition is falsy, then if we have Some else branch eval
            // that, otherwise return an empty value as we fell through.
            match *elseb {
                Some(elsebranch) => return self.interpret_stmt(elsebranch),
                None => {
                    return Ok(Value::Empty);
                }
            }
        }
    }

    fn interpret_assignment(&mut self, name: Token, value: Expr) -> Result<Value, String> {
        let val = self.interpret_expr(value)?;
        self.environment.assign(name, &val)?;
        Ok(val)
    }

    fn interpret_binary(&mut self, left: Expr, oper: Token, right: Expr) -> Result<Value, String> {
        let left = self.interpret_expr(left)?;
        let right = self.interpret_expr(right)?;

        let l = match left {
            Value::Number(x) => x,
            // String concatenation is done with + because that's what everything
            // else uses. As such we need to handle this without tying ourselves
            // in knots or making rustc angry about types, so we do it here.
            Value::String(ref x) => match right {
                Value::String(y) => {
                    return Ok(Value::String(format!("{}{}", x, y)));
                }
                _ => {
                    let emsg = format!("Attempted to concatenate values of incompatable types. left: {:?} right {:?}", x, left);
                    return Err(emsg);
                }
            },
            _ => {
                let emsg = format!("Attempted to apply a binary operation {:?} to {:?} and {:?} but lvalue ({:?}) is not a number!",
                                   oper, left, right, left);
                return Err(emsg);
            }
        };

        let r = match right {
            Value::Number(x) => x,
            _ => {
                let emsg = format!("Attempted to apply a binary operation {:?} to {:?} and {:?} but rvalue ({:?}) is not a number!",
                                   oper, left, right, right);
                return Err(emsg);
            }
        };

        match oper.ttype {
            TokenType::Minus => {
                return Ok(Value::Number(l - r));
            }
            TokenType::Slash => {
                if l == 0.0 || r == 0.0 {
                    let emsg =
                        format!("Attempted to divide by zero!. Expression was {} / {}", l, r);
                    return Err(emsg);
                } else {
                    return Ok(Value::Number(l / r));
                }
            }
            TokenType::Star => {
                return Ok(Value::Number(l * r));
            }
            TokenType::Plus => {
                return Ok(Value::Number(l + r));
            }
            TokenType::EqualEqual => {
                return Ok(Value::Bool(self.is_equal(left, right)));
            }
            TokenType::BangEqual => {
                return Ok(Value::Bool(!self.is_equal(left, right)));
            }
            TokenType::Greater => {
                return Ok(Value::Bool(l > r));
            }
            TokenType::GreaterEqual => {
                return Ok(Value::Bool(l >= r));
            }
            TokenType::Less => {
                return Ok(Value::Bool(l < r));
            }
            TokenType::LessEqual => {
                return Ok(Value::Bool(l <= r));
            }
            _ => {
                let msg = format!(
                    "Attempted to evaluate an invalid binary expression. {:?} {:?} {:?}",
                    left, oper, right
                );
                return Err(msg);
            }
        }
    }

    /// Interprets function calls.
    /// Any returned Value will be evaluated as an RHS in the case of a variable
    /// definition statement.
    ///
    /// # Example
    /// ```
    /// ligamen a = horologium();
    /// scribo a;
    /// ```
    /// In this case, a will contain the current unix time stamp.
    fn interpret_call(
        &mut self,
        callee: Expr,
        _paren: Token,
        args: Vec<Expr>,
    ) -> Result<Value, String> {
        let evaled = match callee {
            Expr::Variable(ref v) => match self.environment.get(v.clone()) {
                Some(f) => f,
                None => {
                    let emsg = format!("Tried to call undefined function {}", v.lexeme);
                    return Err(emsg);
                }
            },
            _ => {
                let emsg = format!(
                    "Tried to call {} as a function, when it is a {}.",
                    callee.clone(),
                    callee
                );
                return Err(emsg);
            }
        };
        let mut evaledArgs: Vec<Value> = Vec::new();
        for arg in args {
            evaledArgs.push(self.interpret_expr(arg)?);
        }
        match evaled {
            Value::NativeFn(f) => match f.call(self, evaledArgs) {
                Ok(retval) => {
                    //dbg!(self.environment.clone());
                    self.retval = retval.clone();
                    match retval {
                        Some(rv) => {
                            return Ok(rv)
                        },
                        None => {
                            return Ok(Value::Empty)
                        },
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            },
            Value::UserFn(f) => match f.call(self, evaledArgs) {
                Ok((mutated, retval)) => {
                    self.environment = mutated;
                    self.retval = retval.clone();
                    match retval {
                        Some(rv) => {
                            return Ok(rv);
                        }
                        None => {
                            return Ok(Value::Empty);
                        }
                    }
                }
                Err(failure) => {
                    let emsg = failure.1;
                    return Err(emsg);
                }
            },
            _ => {
                let emsg = format!(
                    "{} is neither a function, nor a language construct, it is a {}",
                    callee, evaled
                );
                return Err(emsg);
            }
        }
    }

    fn interpret_literal(&mut self, value: Literal) -> Result<Value, String> {
        match value {
            Literal::Number(x) => Ok(Value::Number(x)),
            Literal::StrLit(x) => Ok(Value::String(x)),
            Literal::Bool(x) => Ok(Value::Bool(x)),
            Literal::Empty => Ok(Value::Empty),
        }
    }

    fn interpret_logical(
        &mut self,
        left: Expr,
        operator: Token,
        right: Expr,
    ) -> Result<Value, String> {
        let left = self.interpret_expr(left)?;

        // If we can short-circuit, then do.
        match operator.ttype {
            TokenType::Or => {
                if Interpreter::is_truthy(left.clone()) {
                    return Ok(left);
                } else if !Interpreter::is_truthy(left.clone()) {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if Interpreter::is_truthy(left.clone()) {
                    return Ok(left);
                } else {
                    return Ok(self.interpret_expr(right)?);
                }
            }
            _ => {}
        }
        self.interpret_expr(right)
    }

    fn interpret_unary(&mut self, operator: Token, right: Expr) -> Result<Value, String> {
        // Evaluate the operand that we are applying the operator too.
        let evaledright = self.interpret_expr(right.clone())?;

        // Match the token type of the operator so we know what kind of maths
        // we need to apply.
        match operator.ttype {
            TokenType::Minus => {
                match evaledright {
                    Value::Number(x) => {
                        return Ok(Value::Number(-x));
                    }
                    _ => {
                        // If we somehow got to the point where a unary oper
                        // is being applied to something other than a number we should
                        // probably let the user know and be scared.
                        //let emsg = format!("Attempted to interpret a unary operation with the invalid operator {:?}", operator);
                        let emsg = format!("Attempted to apply unary operator '{}' to expression {}, which is invalid.", operator.lexeme, right);
                        return Err(emsg);
                    }
                }
            }
            TokenType::Bang => {
                match evaledright {
                    // We take our truthyness and falsyness from ruby.
                    // False and Empty are falsy, everything else is truthy.
                    Value::Empty => {
                        // Empty is fals-y so eval it to true
                        return Ok(Value::Bool(true));
                    }
                    Value::Bool(x) => {
                        // If the value we evaluated earlier is a boolean
                        // then negate that
                        return Ok(Value::Bool(!x));
                    }
                    _ => {
                        // Anything other than Empty or Bool::False is truthy
                        // so our negation is false.
                        return Ok(Value::Bool(false));
                    }
                }
            }
            _ => {
                let errormsg = format!(
                    "Attempted to interpret unary operation with expr {:?}",
                    right.clone()
                );
                return Err(errormsg);
            }
        }
    }

    fn interpret_var_expr(&mut self, name: Token) -> Result<Value, String> {
        match self.environment.get(name.clone()) {
            Some(x) => {
                return Ok(x);
            }
            None => {
                dbg!(self.environment.clone());
                let emsg = format!(
                    "Tried to access undefined variable with the name {}",
                    name.lexeme.clone()
                );
                return Err(emsg);
            }
        }
    }

    fn is_equal(&mut self, lv: Value, rv: Value) -> bool {
        if lv == Value::Empty && rv == Value::Empty {
            return true;
        }
        if lv == Value::Empty {
            return false;
        }
        return lv == rv;
    }

    // Assoc functions. We dont need to take self for these so, we'll avoid
    // the java-ism of making them methods.

    /// Truthyness is wheater a value is treated as true, or false.
    /// Booleans evaluate to themselves, Empty types are false, everything
    /// else is truth-y. This is shamelessly inspired by how our zen masters
    /// ruby do this.
    fn is_truthy(val: Value) -> bool {
        match val {
            // The truthyness of a bool is itself.
            Value::Bool(x) => {
                return x;
            }
            // Empty types are fals-y
            Value::Empty => {
                return false;
            }
            // All other types are truthy.
            _ => {
                return true;
            }
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use super::Value;
    use super::*;
    use crate::ast::ASTNode;
    use crate::environment::Environment;
    use crate::parser::Parser;
    use crate::scanner::*;
    use crate::token::*;

    #[test]
    fn addition_test() {
        let test_str: String = "5 + 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Number(15.0);
        assert!(result == expected);
    }

    #[test]
    fn subtraction_test() {
        let test_str: String = "10 - 5;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Number(5.0);
        assert!(result == expected);
    }

    #[test]
    fn multiplication_test() {
        let test_str: String = "3 * 5;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Number(15.0);
        assert!(result == expected);
    }

    #[test]
    fn division_test() {
        let test_str: String = "100 / 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Number(10.0);
        assert!(result == expected);
    }

    #[test]
    fn string_concat() {
        let test_str: String = "\"Hello, \" + \"World!\";".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::String("Hello, World!".to_string());
        assert!(result == expected);
    }

    #[test]
    fn equality() {
        let test_str: String = "10 == 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn inequality() {
        let test_str: String = "100 != 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn gt() {
        let test_str: String = "100 > 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn geq() {
        let test_str: String = "10 >= 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn lt() {
        let test_str: String = "100 < 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(false);
        assert!(result == expected);
    }

    #[test]
    fn leq() {
        let test_str: String = "10 <= 10;".to_string();
        let parsed = process(test_str);
        let result = extract_retval(parsed);
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn bind_test() {
        let test = String::from(
            r#"
          ligamen testVal = 1337;
        "#,
        );
        let parsed = process(test);
        let env = eval_and_extract_state(parsed);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            3,
        );
        let expected = Value::Number(1337.0);
        assert!(env.get(symbol).unwrap() == expected);
    }

    #[test]
    fn shadow_test() {
        let test = String::from(
            r#"
          ligamen testVal = "BAD";
          ligamen testVal = 1337;
        "#,
        );
        let parsed = process(test);
        let env = eval_and_extract_state(parsed);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            3,
        );
        let expected = Value::Number(1337.0);
        assert!(env.get(symbol).unwrap() == expected);
    }

    #[test]
    fn globals() {
        let test: String = String::from(
            r#"ligamen testVal = 0;
        incantatio fun() {
          testVal = 1337;
        }
        fun();"#,
        );
        let parsed = process(test);
        let env = eval_and_extract_state(parsed);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            3,
        );
        let expected = Value::Number(1337.0);
        assert!(env.get(symbol).unwrap() == expected);
    }

    #[test]
    fn assignment_test() {
        let test = String::from(
            r#"
          ligamen testVal = 0;
          testVal = 1337;
        "#,
        );
        let parsed = process(test);
        let env = eval_and_extract_state(parsed);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            3,
        );
        let expected = Value::Number(1337.0);
        assert!(env.get(symbol).unwrap() == expected);
    }

    #[test]
    fn fundef() {
        let test: String = String::from(
            r#"incantatio fun() {
          "Hi";
        }"#,
        );
        let parsed = process(test);
        let env = eval_and_extract_state(parsed);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("fun"),
            Literal::Empty,
            1,
        );
        let result = env.get(symbol.clone());
        let expected = Some(Value::UserFn(UserFunction::new(
            symbol,
            vec![Stmt::Expression(Expr::Literal(Literal::StrLit(
                String::from("Hi"),
            )))],
            vec![],
        )));
        assert!(result == expected);
    }

    #[test]
    fn bool_and() {
        let test = String::from("ligamen testVal = verum et verum;");
        let parsed = process(test);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            1,
        );
        let expected = eval_and_expect(parsed, Some(Value::Bool(true)), symbol);
        assert!(expected == true);
    }

    #[test]
    fn bool_or() {
        let test = String::from("ligamen testVal = verum vel verum;");
        let parsed = process(test);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            1,
        );
        let expected = eval_and_expect(parsed, Some(Value::Bool(true)), symbol);
        assert!(expected == true);
    }

    #[test]
    fn truthynes_numbers() {
        let test = String::from("ligamen a = verum; ligamen b = 5; ligamen testVal = ( a et b );");
        let parsed = process(test);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            1,
        );
        let expected = eval_and_expect(parsed, Some(Value::Bool(true)), symbol);
        assert!(expected == true);
    }

    #[test]
    fn truthynes_strings() {
        let test =
            String::from("ligamen a = verum; ligamen b = \"hi\"; ligamen testVal = ( a et b );");
        let parsed = process(test);
        let symbol = Token::new(
            TokenType::Identifier,
            String::from("testVal"),
            Literal::Empty,
            1,
        );
        let expected = eval_and_expect(parsed, Some(Value::Bool(true)), symbol);
        assert!(expected == true);
    }

    fn process(testcase: String) -> Vec<ASTNode> {
        let lexed = Scanner::scan(testcase);
        Parser::parse(lexed)
    }

    fn extract_retval(ast: Vec<ASTNode>) -> Value {
        let mut i: Interpreter = Interpreter::new();
        i.interpret(ast).unwrap().get(0).unwrap().clone()
    }

    fn eval_and_extract_state(ast: Vec<ASTNode>) -> Environment {
        let mut i: Interpreter = Interpreter::new();
        let x = i.interpret(ast);
        dbg!(i.environment.clone());
        println!("{:?}", x);
        i.environment
    }

    fn eval_and_expect(ast: Vec<ASTNode>, expected: Option<Value>, symb: Token) -> bool {
        let state = eval_and_extract_state(ast);
        state.get(symb) == expected
    }
}
