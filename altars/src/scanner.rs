use std::collections::HashMap;

use substring::Substring;

use lazy_static::*;

use crate::literals::Literal;
use crate::token::Token;
use crate::tokentype::TokenType;

/// Given raw source code, this struct provides tools to lex/tokenize it.
pub struct Scanner {
    /// Input raw source code
    src: String,
    /// List of tokens that have been lexed so far.
    tokens: Vec<Token>,
    /// Starting position of the current token
    start: usize,
    /// Index of the current character under the cursor.
    current: usize,
    /// Line counter for better error reporting.
    line: usize,
}

/// Represent error conditions in the lexer.
enum ScanError {
    UknownCharacter((usize, char)),
    UnterminatedBlockComment(usize),
    UnterminatedString(usize),
    NumParseError((usize, String)),
}

lazy_static! {
    /// This is a list of all reserved keywords in daemonica.
    ///
    /// We use lazy-static so we only need to initialize this once, and then we
    /// can access it throughout the code base as a safe global.
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
    /// Given some source, produce a new Scanner instance.
    pub fn new(src: String) -> Scanner {
        Scanner {
            src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Convinience function. Provides a way to just do execution without needing
    /// a sepperate instantiation of the lexer.
    pub fn scan(src: String) -> Vec<Token> {
        let mut s = self::Scanner::new(src);
        s.scan_tokens()
    }

    /// Main loop. This contains the flow control for the lexing process.
    ///
    /// Continue lexing until we either hit EOF, or until we encounter some error.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // This allows us for some flow control wherein if we do find an invalid
        // section, then we keep on going until we reach the end, and only THEN
        // panic. This lets us catch all the lexing errors at once.
        let mut had_error: bool = false;
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
                        had_error = true;
                    }
                    ScanError::UnterminatedBlockComment(x) => {
                        eprintln!(
                            "Lexer encountered unterminated block comment ('/* */') at line {}",
                            x
                        );
                        had_error = true;
                    }
                    ScanError::UnterminatedString(x) => {
                        eprintln!("Lexer encountered unterminated String at line {}", x);
                        had_error = true;
                    }
                    ScanError::NumParseError(x) => {
                        eprintln!("Lexer encountered invalid number {} at line {}", x.1, x.0);
                        had_error = true;
                    }
                },
            }
        }
        // Push our EOF token when we break the loop.
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Empty,
            self.line,
        ));
        // if we produced an error, we need to report it, as well as terminate.
        // panic does both.
        match had_error {
            true => {
                panic!("Invalid tokens. Terminating.");
            }
            false => {
                return self.tokens.clone();
            }
        }
    }

    /// Lex tokens. Implemented as a big match statement.
    ///
    /// If we see known tokens, we add them to the list of tokens we have lexed.
    /// For certain values such as \"\" which represent strings or for numeric values
    /// we branch out to helper functions to lex them.
    ///
    /// Most of the time this lexer operates as 1 lookahead, in order to match
    /// 2-char long statements such as != and ==.
    ///
    /// Join the One Big ~~Union~~ Match Statement.
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
                        self.next();
                        // if the next token is \n then we need to increment
                        // the line counter for error reporting
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        // Error out if the user forgot to terminate their block
                        // comment
                        if self.at_end() {
                            return Err(ScanError::UnterminatedBlockComment(self.line));
                        }
                    }
                    self.next();
                    self.next();
                    self.next();
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
                // Base case, anything else that falls through here is treated
                // as an identifier.
                if c.is_ascii_alphanumeric() {
                    self.lex_identifier();
                } else {
                    return Err(ScanError::UknownCharacter((self.line, c)));
                }
            }
        }
        Ok(())
    }

    /// Keep peeking until our next token is a closing ", adding each char to a string.
    /// If the "" is unterminated, we report an error. Otherwise we return
    /// the lexed string.
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

    /// Simmialr to our string function, keep peeking while we have numeric parts.
    ///
    /// We need some special handling since we allow floats. As such we look 2-ahead
    /// after we break out of our first loop. If we have a number after it, we know that
    /// everything after is the fractional part.
    /// Another annoying edge case where we need to be 2-lookahead.
    fn lex_number(&mut self) -> Result<(), ScanError> {
        while self.peek().is_numeric() {
            self.next();
        }

        // Do we have a decimal point?
        if self.peek() == '.' && self.peek_n(1).is_numeric() {
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
            Err(v) => {
                eprintln!("{}", v);
                return Err(ScanError::NumParseError((self.line, text.to_string())));
            }
        }
    }

    /// Lexes identifiers for fns/vars/keywords etc.
    ///
    /// If we get a keyword we grab it's raw value from our keywords list
    /// if we dont, then we just put the lexed value as a Identifier token.
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

    /// Look current+n ahead, without consuming the token.
    fn peek_n(&self, n: usize) -> char {
        if self.at_end() || self.current + n >= self.src.len() {
            return '\0';
        } else {
            return self.src.chars().nth(self.current + n).unwrap();
        }
    }

    /// Lookahead 1-char without peeking.
    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.src.chars().nth(self.current).unwrap();
        }
    }

    // Probably wont need this, but having n char lookahead for debugging
    // can be pretty useful.
    /// Used for debugging. Allows for n-lookahead lexing. Not designed for
    /// actual consumption but is good for fixing bugs.
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

    /// Predicate function. If expect is the next char true, else false.
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

    /// Add a processed token to our token vec
    /// we grab the text of the current lexeme, and create a token for it.
    fn add_token(&mut self, ttype: TokenType) {
        let text = self.src.substring(self.start, self.current).to_string();
        self.tokens
            .push(Token::new(ttype, text, Literal::Empty, self.line))
    }

    /// Add a procesesd token, that has a literal value to our vec.
    fn add_token_with_literal(&mut self, ttype: TokenType, lit: Literal) {
        let text = self.src.substring(self.start, self.current).to_string();
        self.tokens.push(Token::new(ttype, text, lit, self.line))
    }

    /// Consume the next character in the source file and return it.
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
        let test: String = "5;".to_string();
        let mut s: Scanner = Scanner::new(test);
        let tokens = s.scan_tokens();
        let five = Token::new(TokenType::Number, "5".to_string(), Literal::Number(5.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_string(), Literal::Empty, 1);
        let eof = Token::new(TokenType::EOF, "".to_string(), Literal::Empty, 1);
        let expected = vec![five, semicolon, eof];
        assert!(expected == tokens);
    }

    #[test]
    fn parenthesized_exprs() {
        let test: String = "5 + (3 * (8));".to_string();
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
        let mut s: Scanner = Scanner::new(test);
        let tokens = s.scan_tokens();
        assert!(expected == tokens);
    }

    #[test]
    fn and_test() {
        let test: String = "et;".to_string();
        let expected = vec![
            Token::new(TokenType::And, "et".to_string(), Literal::Empty, 1),
            get_semicolon(1),
            get_eof(1),
        ];
        let mut s: Scanner = Scanner::new(test);
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
            get_semicolon(1),
            get_eof(1),
        ];
        let mut s: Scanner = Scanner::new(velocity);
        let tokens = s.scan_tokens();
        assert!(expected == tokens);
    }

    #[test]
    fn scan_direct() {
        let test = String::from("ligamen a = 5;");
        let results = crate::scanner::Scanner::scan(test);
        let expected = vec![
            Token {
                ttype: TokenType::Var,
                lexeme: "ligamen".to_string(),
                literal: Literal::Empty,
                line: 1,
            },
            Token {
                ttype: TokenType::Identifier,
                lexeme: "a".to_string(),
                literal: Literal::Empty,
                line: 1,
            },
            Token {
                ttype: TokenType::Equal,
                lexeme: "=".to_string(),
                literal: Literal::Empty,
                line: 1,
            },
            Token {
                ttype: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Literal::Number(5.0),
                line: 1,
            },
            get_semicolon(1),
            get_eof(1),
        ];
        assert!(expected == results);
    }

    #[test]
    fn comments() {
        let linecomment = String::from("// This is a line comment.");
        let expected = vec![get_eof(1)];
        let res = crate::scanner::Scanner::scan(linecomment);
        assert!(res == expected);
        let expected = vec![get_eof(2)];
        let blockcomment = String::from("/*Block comments\nCan Span multiple lines.*/");
        let res = crate::scanner::Scanner::scan(blockcomment);
        assert!(res == expected);
    }

    #[test]
    fn non_fractional_num() {
        let test = String::from("3.0;");
        let result = crate::scanner::Scanner::scan(test);
        let expected = vec![
            Token::new(
                TokenType::Number,
                String::from("3.0"),
                Literal::Number(3.0),
                1,
            ),
            get_semicolon(1),
            get_eof(1),
        ];
        assert!(result == expected);
    }

    #[test]
    fn fractional_numbers() {
        let test = String::from("3.14159;");
        let result = crate::scanner::Scanner::scan(test);
        let expected = [
            vec![Token::new(
                TokenType::Number,
                String::from("3.14159"),
                Literal::Number(3.14159),
                1,
            )],
            get_end(1),
        ]
        .concat();
        assert!(result == expected);
    }

    #[test]
    #[should_panic]
    /// We only support ASCII-alphanumeric because whilst it's acceptable in rust
    /// having to discriminate between identifiers and such is kind of a mess.
    /// This is also what most people expect, for better or for worse.
    fn scan_invalid_chars() {
        let test = String::from("こんばんわ");
        crate::scanner::Scanner::scan(test);
    }

    #[test]
    #[should_panic]
    fn unterminated_string() {
        let test = String::from("\"This is not terminated.");
        crate::scanner::Scanner::scan(test);
    }

    #[test]
    #[should_panic]
    fn unterminated_block_comment() {
        let test = String::from("/*ligamen a = \"test\";");
        crate::scanner::Scanner::scan(test);
    }

    fn get_end(line: usize) -> Vec<Token> {
        vec![get_semicolon(line), get_eof(line)]
    }

    fn get_eof(line: usize) -> Token {
        Token::new(TokenType::EOF, "".to_string(), Literal::Empty, line)
    }

    fn get_semicolon(line: usize) -> Token {
        Token {
            ttype: TokenType::Semicolon,
            lexeme: ";".to_string(),
            literal: Literal::Empty,
            line,
        }
    }
}
