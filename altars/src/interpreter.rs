use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Value;
use crate::ast::Visitor;
use crate::literals::Literal;
use crate::token::Token;
use crate::tokentype::TokenType;

pub struct Interpreter;
//impl<T> Visitor<T> for Interpreter {
impl Interpreter {
    pub fn interpret_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Assign(name, value) => {
                self.interpret_assignment(name, *value)
            },
            Expr::Binary(left, oper, right) => {
                self.interpret_binary(*left, oper, *right)
            },
            Expr::Call(callee, paren, args) => {
                todo!()
            },
            Expr::Get(object, name) => {
                todo!()
            },
            Expr::Grouping(expression) => {
                // Destructure the expression and recursivley interpret it's
                // subexpressions
                self.interpret_expr(*expression)
            },
            Expr::Literal(value) => {
                self.interpret_literal(value)
            },
            Expr::Logic(left, operator, right) => {
                self.interpret_logical(*left, operator, *right)
            },
            Expr::Set(object, name, value) => {
                todo!()
            },
            Expr::This(keyword) => {
                todo!()
            },
            Expr::Unary(operator, right) => {
                self.interpret_unary(operator, *right)
            },
            Expr::Variable(name) => {
                todo!()
            },
        }
    }

    fn interpret_assignment(&mut self, name: Token, value: Expr) -> Result<Value, String> {
        todo!()
    }

    fn interpret_binary(&mut self, left: Expr, oper: Token, right: Expr) -> Result<Value, String> {
        let left = self.interpret_expr(left)?;
        let right = self.interpret_expr(right)?;

        let l = match left {
            Value::Number(x) => x,
            // String concatenation is done with + because that's what everything
            // else uses. As such we need to handle this without tying ourselves
            // in knots or making rustc angry about types, so we do it here.
            Value::String(ref x) => {
                match right {
                    Value::String(y) => {
                        return Ok(Value::String(format!("{}{}", x, y)));
                    }
                    _ => {
                        let emsg = format!("Attempted to concatenate values of incompatable types. left: {:?} right {:?}", x, left);
                        return Err(emsg);
                    }
                }
            }
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
            },
            TokenType::Slash => {
                if l == 0.0 || r == 0.0 {
                    let emsg = format!("Attempted to divide by zero!. Expression was {} / {}", l, r);
                    return Err(emsg);
                } else {
                    return Ok(Value::Number(l / r));
                }
            },
            TokenType::Star => {
                return Ok(Value::Number(l * r));
            },
            TokenType::Plus => {
                return Ok(Value::Number(l + r));
            },
            TokenType::EqualEqual => {
                return Ok(Value::Bool(self.is_equal(left, right)));
            },
            TokenType::BangEqual => {
                return Ok(Value::Bool(!self.is_equal(left, right)));
            },
            TokenType::Greater => {
                return Ok(Value::Bool(l > r));
            },
            TokenType::GreaterEqual => {
                return Ok(Value::Bool(l >= r));
            },
            TokenType::Less => {
                return Ok(Value::Bool(l < r));
            },
            TokenType::LessEqual => {
                return Ok(Value::Bool(l <= r));
            }
            _ => {
                let msg = format!("Attempted to evaluate an invalid binary expression. {:?} {:?} {:?}", left, oper, right);
                return Err(msg);
            }
        }
    }

    fn interpret_call(&mut self, callee: Expr, paren: Token, args: Vec<Expr>) -> Result<Value, String> {
        todo!()
    }

    fn interpret_get(&mut self, object: Expr, name: Token) -> Result<Value, String> {
        todo!()
    }

    fn interpret_grouping(&mut self, expression: Expr) -> Result<Value, String> {
        todo!()
    }

    fn interpret_literal(&mut self, value: Literal) -> Result<Value, String> {
        match value {
            Literal::Number(x) => {
                Ok(Value::Number(x))
            },
            Literal::StrLit(x) => {
                Ok(Value::String(x))
            },
            Literal::Bool(x) => {
                Ok(Value::Bool(x))
            },
            Literal::Empty => {
                Ok(Value::Empty)
            },
        }
    }

    fn interpret_logical(&mut self, left: Expr, operator: Token, right: Expr) -> Result<Value, String> {
        let left = self.interpret_expr(left)?;
        let right = self.interpret_expr(right)?;
        todo!()
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
                    },
                    _ => {
                        // If we somehow got to the point where a unary oper
                        // is being applied to something other than a number we should
                        // probably let the user know and be scared.
                        //let emsg = format!("Attempted to interpret a unary operation with the invalid operator {:?}", operator);
                        let emsg = format!("Attempted to apply unary operator '{}' to expression {}, which is invalid.", operator.lexeme, right);
                        return Err(emsg);
                    }
                }
            },
            TokenType::Bang => {
                match evaledright {
                    // We take our truthyness and falsyness from ruby.
                    // False and Empty are falsy, everything else is truthy.
                    Value::Empty => {
                        // Empty is fals-y so eval it to true
                        return Ok(Value::Bool(true));
                    },
                    Value::Bool(x) => {
                        // If the value we evaluated earlier is a boolean
                        // then negate that
                        return Ok(Value::Bool(!x));
                    },
                    _ => {
                        // Anything other than Empty or Bool::False is truthy
                        // so our negation is false.
                        return Ok(Value::Bool(false));
                    }
                }
            }
            _ => {
                let errormsg = format!("Attempted to interpret unary operation with expr {:?}", right.clone());
                return Err(errormsg);
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
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::token::*;
    use crate::scanner::*;
    use super::Interpreter;
    use super::Value;

    #[test]
    fn addition_test() {
        let test_str: String = "5 + 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Number(15.0);
        assert!(result == expected);
    }

    #[test]
    fn subtraction_test() {
        let test_str: String = "10 - 5;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Number(5.0);
        assert!(result == expected);
    }

    #[test]
    fn multiplication_test() {
        let test_str: String = "3 * 5;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Number(15.0);
        assert!(result == expected);
    }

    #[test]
    fn division_test() {
        let test_str: String = "100 / 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Number(10.0);
        assert!(result == expected);
    }

    #[test]
    fn string_concat() {
        let test_str: String = "\"Hello, \" + \"World!\"".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::String("Hello, World!".to_string());
        assert!(result == expected);
    }

    #[test]
    fn equality() {
        let test_str: String = "10 == 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn inequality() {
        let test_str: String = "100 != 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn gt() {
        let test_str: String = "100 > 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn geq() {
        let test_str: String = "10 >= 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(true);
        assert!(result == expected);
    }

    #[test]
    fn lt() {
        let test_str: String = "100 < 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(false);
        assert!(result == expected);
    }

    #[test]
    fn leq() {
        let test_str: String = "10 <= 10;".to_string();
        let mut s: Scanner = Scanner::new(test_str);
        let tokens = s.scan_tokens();
        let mut p: Parser = Parser::new(tokens);
        let parsed = p.parse();
        let mut i: Interpreter = Interpreter;
        let result = i.interpret_expr(parsed).unwrap();
        let expected = Value::Bool(true);
        assert!(result == expected);
    }
}
