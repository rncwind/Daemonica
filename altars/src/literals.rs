//! AST Level representation of Literal Values
use core::fmt;
use std::fmt::Display;

/// Represent Literal Values that we parse.
///
/// Conventionally these things are stored as void*s or Objects. (un)fortunatley
/// rust takes inspiration from ML so all of our types need to be concrete.
/// To handle the fact we have a few different literals that can exist we
/// wrap them inside a union as variants.
///
/// This gives us the illusion that we are
/// only refering to some Literal type, when it contains some value.
/// As a wise man once said, there is power in a union.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Number(f64),
    StrLit(String),
    Bool(bool),
    Empty,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(x) => {
                write!(f, "{}", x)
            }
            Literal::StrLit(x) => {
                write!(f, "{}", x)
            }
            Literal::Empty => {
                write!(f, "Empty")
            }
            Literal::Bool(x) => {
                write!(f, "{}", x)
            }
        }
    }
}
