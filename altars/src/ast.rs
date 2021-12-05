//! Datatypes that are present inside our generated ASTs.
use std::fmt::{self, Display};

use crate::literals::Literal;
use crate::nativefn::NativeFn;
use crate::token::Token;
use crate::userfunction::UserFunction;

/// Wraps either an Expr or an Stmt in one type so that we can treat them generically
/// until we actually have to destructure the type for Interpretation.
#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    ExprNode(Expr),
    StmtNode(Stmt),
}

/// Each statement or expression is represented as an algebraic variant that
/// contains it's constituent parts.
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class(Token, Vec<Stmt>),
    Expression(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    // GlobalFunction.
    //Function(Token, Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    Return(Token, Option<Expr>),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Print(Expr),
}

/// Simmilar to the Stmt, we treat each Expression as an algebraic variant of this
/// enum.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    //Call(Box<Expr>, Token),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logic(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

/// We need some way to store state inside the interpreter. This is how.
///
/// Once again, we use ADTs in order to define the possible variants of a Value.
/// This can be many things from stuff exposed to the user, such as numbers, booleans
/// and strings, to the Empty type (No nulls here!), or the internal representations
/// of how we move about and store functions.
///
/// If it's data, and it's (nominally) mutable at runtime, it's probably a Value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    NativeFn(NativeFn),
    UserFn(UserFunction),
    Empty,
}

/// This allows us to convert from Literals to values.
impl From<Literal> for Value {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Number(v) => {
                return Value::Number(v);
            }
            Literal::StrLit(v) => {
                return Value::String(v);
            }
            Literal::Bool(v) => {
                return Value::Bool(v);
            }
            Literal::Empty => {
                return Value::Empty;
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(x) => {
                write!(f, "{}", x)
            }
            Value::Bool(x) => {
                write!(f, "{}", x)
            }
            Value::String(x) => {
                write!(f, "{}", x)
            }
            Value::Empty => {
                write!(f, "Empty")
            }
            Value::NativeFn(x) => {
                write!(f, "{}", x)
            }
            Value::UserFn(x) => {
                write!(f, "{:?}", x)
            } //Value::Symbol(n, v) => {
              //write!(f, "{} = {}", n, *v)
              //}
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                let mut rv = String::from("BLOCK:( ");
                for stmt in stmts {
                    rv = format!("{} {}", rv, stmt);
                }
                write!(f, "{} )", rv)
            }
            Stmt::Class(token, body) => {
                let mut rv = String::from("CLASS: (");
                rv = format!("{} {}", rv, token);
                for stmt in body {
                    rv = format!("{} {}", rv, stmt);
                }
                write!(f, "{} )", rv)
            }
            Stmt::Expression(expr) => {
                write!(f, "EXPR: ( {} )", expr)
            }
            Stmt::Function(name, body, params) => {
                let mut rv = format!("Fn {} ({:?}): Body: {{", name, params);
                for stmt in body {
                    rv = format!("{} {}", rv, stmt);
                }
                rv = format!("{} }}", rv);
                write!(f, "{}", rv)
            }
            Stmt::If(_, _, _) => todo!(),
            Stmt::Return(_, _) => todo!(),
            Stmt::Var(_, _) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::Print(_) => todo!(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(_, _, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
            Expr::Get(_, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Literal(x) => {
                write!(f, "\"{}\"", x)
            }
            Expr::Logic(_, _, _) => todo!(),
            Expr::Set(_, _, _) => todo!(),
            Expr::This(_) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Variable(_) => todo!(),
        }
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::ExprNode(x) => {
                write!(f, "{}", x)
            }
            ASTNode::StmtNode(x) => {
                write!(f, "{}", x)
            }
        }
    }
}
