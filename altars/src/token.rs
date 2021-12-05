//! Token and AST-Level Symbol information
use std::fmt;
use std::fmt::Display;

use crate::literals::Literal;
use crate::tokentype::TokenType;

/// Our Token type represents the parsed Tokens that we got from our scanner.
///
/// More generally, they are usefull as they store their internal type values, and their
/// names. As such, we use these as Symbols as well.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TOKEN( Type: {}, Lexeme: {}, Literal: {}, line: {} )",
            self.ttype, self.lexeme, self.literal, self.line
        )
    }
}
