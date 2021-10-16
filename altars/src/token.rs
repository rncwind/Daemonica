use crate::tokentype::TokenType;
use crate::literals::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token{
            ttype, lexeme, literal, line
        }
    }
}
