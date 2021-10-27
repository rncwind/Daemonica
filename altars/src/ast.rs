use std::fmt::{self, Display};

use crate::literals::Literal;
use crate::token::Token;

/// We can wrap either `Stmt` or a `Expr` inside an ASTNode so we can treat them
/// generically, up until the pattern matching stge
pub enum ASTNode {
    ExprNode(Expr),
    StmtNode(Stmt),
}

/// Each statement or expression is represented as a dumb algebraic type that
/// contains it's constituent parts.
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class(Token, Vec<Stmt>),
    Expression(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Stmt>),
    Return(Token, Expr),
    Var(Token, Expr),
    While(Expr, Box<Stmt>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
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

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Empty,
}

/// This is a trait that allows any given struct to implement the visitor pattern
/// for any of statement or expression.
pub trait Visitor<T> {
    fn visit_stmt(&mut self, x: &Stmt) -> T;
    fn visit_expr(&mut self, x: &Expr) -> T;
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
            Stmt::Function(name, params, body) => {
                let mut rv = String::from("FN : ( ");
                rv = format!("{} name: {} params: ( ", rv, name);
                for param in params {
                    rv = format!("{} {}", rv, param);
                }
                rv = format!("{} ) body: ( ", rv);
                for stmt in body {
                    rv = format!("{} {}", rv, stmt);
                }
                rv = format!("{} )", rv);
                write!(f, "{}", rv)
            }
            Stmt::If(_, _, _) => todo!(),
            Stmt::Return(_, _) => todo!(),
            Stmt::Var(_, _) => todo!(),
            Stmt::While(_, _) => todo!(),
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
            },
            Expr::Logic(_, _, _) => todo!(),
            Expr::Set(_, _, _) => todo!(),
            Expr::This(_) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Variable(_) => todo!(),
        }
    }
}
