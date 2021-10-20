use crate::{ast::{Expr, Stmt, Visitor}, literals::Literal, token::Token, tokentype::TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// Parse equality expressions (== and !=)
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.expect(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse >, >=, < and <= expressions
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.expect(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// parse + and - expressions
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.expect(vec![
            TokenType::Plus,
            TokenType::Minus,
        ]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse * and / expressions
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.expect(vec![
            TokenType::Star,
            TokenType::Slash,
        ]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        // Check to see if it's a ! or -. If it is ten it's a unary expression
        // so grab the token and recurse un unary to parse te operand.
        if self.expect(vec![
            TokenType::Bang,
            TokenType::Minus
        ]) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary(op, Box::new(right))
        } else {
            // If it's not a unary expr, then we have a primary expression
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.expect(vec![TokenType::False]) {
            return Expr::Literal(Literal::Bool(false));
        }
        if self.expect(vec![TokenType::True]) {
            return Expr::Literal(Literal::Bool(true));
        }
        if self.expect(vec![TokenType::None]) {
            return Expr::Literal(Literal::Empty);
        }
        if self.expect(vec![TokenType::Number]) {
            return Expr::Literal(self.previous().literal);
        }
        if self.expect(vec![TokenType::String]) {
            return Expr::Literal(self.previous().literal);
        }

        if self.expect(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen);
            return Expr::Grouping(Box::new(expr));
        }
        panic!("Somehow bottomed out of Parser::primary")
    }

    // Helper functions that abstract out common logic.

    fn consume(&mut self, ttype: TokenType) -> Token {
        if self.expect(vec![ttype.clone()]) {
            return self.next();
        } else {
            panic!("Was expecting token {:?}, but found {:?} at line {}",
            ttype, self.peek().ttype, self.current);
        }
    }

    /// Given a list of valid tokentypes, see if the next token in the stream is
    /// in that list, and thus valid
    fn expect(&mut self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.check(ttype) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Helper for expect. Produce true if the tokentype is valid, else false
    fn check(&self, ttype: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            return self.peek().ttype == ttype;
        }
    }

    /// Advance to the next token, producing the previous token
    fn next(&mut self) -> Token {
        if self.at_end() == false {
            self.current += 1;
        }
        self.previous()
    }

    /// Lookahead one token.
    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    /// Return true if the next token is EOF.
    fn at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    /// Produce the previous token in the token stream
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}

impl<T> Visitor<T> for Parser {
    fn visit_stmt(&mut self, x: &Stmt) -> T {
        todo!()
    }

    fn visit_expr(&mut self, x: &Expr) -> T {
        todo!()
    }
}
