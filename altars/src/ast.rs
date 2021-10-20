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
#[derive(Clone)]
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

#[derive(Clone, Debug)]
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

/// This is a trait that allows any given struct to implement the visitor pattern
/// for any of statement or expression.
pub trait Visitor<T> {
    fn visit_stmt(&mut self, x: &Stmt) -> T;
    fn visit_expr(&mut self, x: &Expr) -> T;
}
