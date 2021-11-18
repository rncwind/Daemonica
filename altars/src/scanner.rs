use std::collections::HashMap;

use substring::Substring;

use lazy_static::*;

use crate::literals::Literal;
use crate::token::Token;
use crate::tokentype::TokenType;

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

enum ScanError {
    UknownCharacter((usize, char)),
    UnterminatedBlock(usize),
    UnterminatedString(usize),
    NumParseError((usize, String)),
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("et".to_string(), TokenType::And);
        m.insert("vel".to_string(), TokenType::Or);
        m.insert("si".to_string(), TokenType::If);
        m.insert("aliter".to_string(), TokenType::Else);
        m.insert("verum".to_string(), TokenType::True);
        m.insert("mendacium".to_string(), TokenType::False);
        m.insert("incantatio".to_string(), TokenType::Fn);
        m.insert("beneficium".to_string(), TokenType::Return);
        m.insert("enim".to_string(), TokenType::For);
        m.insert("dum".to_string(), TokenType::While);
        m.insert("nihil".to_string(), TokenType::None);
        m.insert("anima".to_string(), TokenType::Self_);
        m.insert("ligamen".to_string(), TokenType::Var);
        m.insert("daemonium".to_string(), TokenType::Class);
        m.insert("cognatio".to_string(), TokenType::Super);
        m.insert("invocabo".to_string(), TokenType::Call);
        m.insert("scribo".to_string(), TokenType::Print);
        return m;
    };
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // This allows us for some flow control wherein if we do find an invalid
        // section, then we keep on going until we reach the end, and only THEN
        // panic. This lets us catch all the lexing errors at once.
        let mut hadError: bool = false;
        while !self.at_end() {
            self.start = self.current;
            // Scan a token and add it to the vec of tokens
            match self.scan_token() {
                // If no errors keep on going
                Ok(_) => {}
                // Error reporting
                Err(x) => match x {
                    ScanError::UknownCharacter(x) => {
                        eprintln!("Scanned invald token {} on line {}", x.1, x.0);
                        hadError = true;
                    }
                    ScanError::UnterminatedBlock(x) => {
                        eprintln!(
                            "Lexer encountered unterminated block comment ('/* */') at line {}",
                            x
                        )
                    }
                    ScanError::UnterminatedString(x) => {
                        eprintln!("Lexer encountered unterminated String at line {}", x)
                    }
                    ScanError::NumParseError(x) => {
                        eprintln!("Lexer encountered invalid number {} at line {}", x.1, x.0);
                    }
                },
            }
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Empty,
            self.line,
        ));
        match hadError {
            true => {
                panic!("Invalid tokens found. Terminating");
            }
            false => {
                return self.tokens.clone();
            }
        }
    }

    fn scan_token(&mut self) -> Result<(), ScanError> {
        let c: char = self.next();
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen);
            }
            ')' => {
                self.add_token(TokenType::RightParen);
            }
            '{' => {
                self.add_token(TokenType::LeftBrace);
            }
            '}' => {
                self.add_token(TokenType::RightBrace);
            }
            ';' => {
                self.add_token(TokenType::Semicolon);
            }
            ',' => {
                self.add_token(TokenType::Comma);
            }
            '.' => {
                self.add_token(TokenType::Dot);
            }
            '*' => {
                self.add_token(TokenType::Star);
            }
            '-' => {
                self.add_token(TokenType::Minus);
            }
            '+' => {
                self.add_token(TokenType::Plus);
            }
            // We need lookahead on these, so we look 1c ahead for thse pesky
            // compoud operations.
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '/' => {
                // Check for divide or line comment
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.next();
                    }
                }
                // Check for block comment
                else if self.match_next('*') {
                    while self.peek() != '*' && self.peek_n(2) != '/' {
                        // if the next token is \n then we need to increment
                        // the line counter for error reporting
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        // Error out if the user forgot to terminate their block
                        // comment
                        if self.at_end() {
                            return Err(ScanError::UnterminatedBlock(self.line));
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => {
                self.lex_string()?;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.lex_number()?;
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            _ => {
                if c.is_ascii_alphabetic() {
                    self.lex_identifier();
                } else {
                    return Err(ScanError::UknownCharacter((self.line, c)));
                }
            }
        }
        Ok(())
    }

    fn lex_string(&mut self) -> Result<(), ScanError> {
        while self.peek() != '"' && self.at_end() == false {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.next();
        }

        if self.at_end() {
            return Err(ScanError::UnterminatedString(self.line));
        }

        // Consume the closing "
        self.next();

        // Slice off the ""
        let text = self
            .src
            .substring(self.start + 1, self.current - 1)
            .to_string();
        self.add_token_with_literal(TokenType::String, Literal::StrLit(text.clone()));

        Ok(())
    }

    fn lex_number(&mut self) -> Result<(), ScanError> {
        while self.peek().is_numeric() {
            self.next();
        }

        // Do we have a decimal point?
        if self.peek() == '.' && self.peek_n(2).is_numeric() {
            // Consume the .
            self.next();
            while self.peek().is_numeric() {
                self.next();
            }
        }

        let text = self.src.substring(self.start, self.current);

        match text.parse::<f64>() {
            Ok(x) => {
                self.add_token_with_literal(TokenType::Number, Literal::Number(x));
                Ok(())
            }
            Err(_) => {
                return Err(ScanError::NumParseError((self.line, text.to_string())));
            }
        }
    }

    fn lex_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.next();
        }

        let text = self.src.substring(self.start, self.current).to_string();
        match KEYWORDS.deref().get(&text) {
            Some(x) => {
                self.add_token(x.clone());
            }
            None => {
                self.add_token(TokenType::Identifier);
            }
        }
    }

    // Lookahead, but dont consume.
    fn peek_n(&self, n: usize) -> char {
        if self.at_end() || self.current + n >= self.src.len() {
            return '\0';
        } else {
            return self.src.chars().nth(self.current + n).unwrap();
        }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.src.chars().nth(self.current).unwrap();
        }
    }

    // Probably wont need this, but having n char lookahead for debugging
    // can be pretty useful.
    fn match_n_ahead(&mut self, expect: char, n: usize) -> bool {
        if self.at_end() || self.current + n >= self.src.len() {
            return false;
        }

        if self.src.chars().nth(self.current + n).unwrap() != expect {
            return false;
        }

        self.current += n;
        return true;
    }

    fn match_next(&mut self, expect: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.src.chars().nth(self.current).unwrap() != expect {
            return false;
        }

        self.current += 1;
        return true;
    }

    // Add a processed token to our token vec
    // we grab the text of the current lexeme, and create a token for it.
    fn add_token(&mut self, ttype: TokenType) {
        let text = self.src.substring(self.start, self.current).to_string();
        self.tokens
            .push(Token::new(ttype, text, Literal::Empty, self.line))
    }

    // Add a procesesd token, that has a literal value to our vec.
    fn add_token_with_literal(&mut self, ttype: TokenType, lit: Literal) {
        let text = self.src.substring(self.start, self.current).to_string();
        self.tokens.push(Token::new(ttype, text, lit, self.line))
    }

    // Consume the next character in the source file and return it.
    fn next(&mut self) -> char {
        let rv = self.src.chars().nth(self.current).unwrap();
        self.current += 1;
        rv
    }

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::*;
    use crate::token::*;
    #[test]
    fn number_literal() {
        let testStr: String = "5;".to_string();
        let mut s: Scanner = Scanner::new(testStr);
        let tokens = s.scan_tokens();
        let five = Token::new(TokenType::Number, "5".to_string(), Literal::Number(5.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_string(), Literal::Empty, 1);
        let eof = Token::new(TokenType::EOF, "".to_string(), Literal::Empty, 1);
        let expected = vec![five, semicolon, eof];
        assert!(expected == tokens);
    }

    #[test]
    fn parenthesized_exprs() {
        let testStr: String = "5 + (3 * (8));".to_string();
        let expected = vec![
            Token::new(TokenType::Number, "5".to_string(), Literal::Number(5.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Empty, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), Literal::Empty, 1),
            Token::new(TokenType::Number, "3".to_string(), Literal::Number(3.0), 1),
            Token::new(TokenType::Star, "*".to_string(), Literal::Empty, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), Literal::Empty, 1),
            Token::new(TokenType::Number, "8".to_string(), Literal::Number(8.0), 1),
            Token::new(TokenType::RightParen, ")".to_string(), Literal::Empty, 1),
            Token::new(TokenType::RightParen, ")".to_string(), Literal::Empty, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), Literal::Empty, 1),
            Token::new(TokenType::EOF, "".to_string(), Literal::Empty, 1),
        ];
        let mut s: Scanner = Scanner::new(testStr);
        let tokens = s.scan_tokens();
        assert!(expected == tokens);
    }

    #[test]
    fn and_test() {
        let testStr: String = "et;".to_string();
        let expected = vec![
            Token::new(TokenType::And, "et".to_string(), Literal::Empty, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), Literal::Empty, 1),
            Token::new(TokenType::EOF, "".to_string(), Literal::Empty, 1),
        ];
        let mut s: Scanner = Scanner::new(testStr);
        let tokens = s.scan_tokens();
        assert!(expected == tokens);
    }

    #[test]
    fn maximal_munch() {
        let velocity: String = "velocity;".to_string();
        let expected = vec![
            Token::new(
                TokenType::Identifier,
                "velocity".to_string(),
                Literal::Empty,
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), Literal::Empty, 1),
            Token::new(TokenType::EOF, "".to_string(), Literal::Empty, 1),
        ];
        let mut s: Scanner = Scanner::new(velocity);
        let tokens = s.scan_tokens();
        assert!(expected == tokens);
    }
}
