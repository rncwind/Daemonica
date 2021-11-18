use crate::{
    ast::{ASTNode, Expr, Stmt, Visitor},
    literals::Literal,
    token::Token,
    tokentype::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut stmts: Vec<ASTNode> = Vec::new();
        while self.at_end() == false {
            //stmts.push(ASTNode::StmtNode(self.statement()));
            stmts.push(ASTNode::StmtNode(self.declaration()));
        }
        return stmts;
    }

    fn declaration(&mut self) -> Stmt {
        if self.maybe_advance(vec![TokenType::Var]) {
            return self.var_decl();
        }
        return self.statement();
    }

    fn statement(&mut self) -> Stmt {
        if self.maybe_advance(vec![TokenType::Print]) {
            return self.parse_print();
        }
        if self.maybe_advance(vec![TokenType::LeftBrace]) {
            return Stmt::Block(self.block());
        }
        if self.maybe_advance(vec![TokenType::While]) {
            return self.while_stmt();
        }
        if self.maybe_advance(vec![TokenType::If]) {
            return self.if_stmt();
        }
        if self.maybe_advance(vec![TokenType::For]) {
            return self.parse_for()
        }
        return self.expression_stmt();
    }

    fn parse_print(&mut self) -> Stmt {
        let val = self.expression();
        self.consume(TokenType::Semicolon);
        return Stmt::Print(val);
    }

    fn parse_for(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen);
        // Parse out the various parts of our for statement, for desugaring
        // in a second

        // Parse our initializer
        let mut initializer: Option<Stmt> = None;
        if self.maybe_advance(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.maybe_advance(vec![TokenType::Var]) {
            initializer = Some(self.var_decl());
        } else {
            initializer = Some(self.expression_stmt());
        }

        // Parse the condition, if it exists.
        let mut cond: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            cond = Some(self.expression());
        }
        self.consume(TokenType::Semicolon);

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression());
        }
        self.consume(TokenType::RightParen);

        let mut body = self.statement();

        match increment {
            Some(inc) => {
                body = Stmt::Block(vec![body, Stmt::Expression(inc)]);
                match cond {
                    Some(_) => {}
                    None => {
                        cond = Some(Expr::Literal(Literal::Bool(true)));
                    }
                }
                body = Stmt::While(cond.unwrap(), Box::new(body));

                match initializer {
                    Some(init) => {
                        body = Stmt::Block(vec![init, body]);
                    }
                    None => {}
                }
            }
            _ => {}
        }
        body
    }

    fn var_decl(&mut self) -> Stmt {
        // Grab the variable name first of all, so we can bind it
        let name = self.consume(TokenType::Identifier);

        // Check if we have an initializer statement or not. If we do we need
        // to parse that out as an expression.
        // otherwise we just bind the name to an empty value
        if self.maybe_advance(vec![TokenType::Equal]) {
            let initializer = Some(self.expression());
            self.consume(TokenType::Semicolon);
            return Stmt::Var(name, initializer);
        } else {
            self.consume(TokenType::Semicolon);
            return Stmt::Var(name, None);
        }
    }

    fn while_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen);
        let condition = self.expression();
        self.consume(TokenType::RightParen);
        let body = self.statement();
        return Stmt::While(condition, Box::new(body));
    }

    fn expression_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon);
        Stmt::Expression(expr)
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.at_end() {
            stmts.push(self.declaration());
        }
        self.consume(TokenType::RightBrace);
        stmts
    }

    fn if_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen);
        let condition = self.expression();
        self.consume(TokenType::RightParen);

        let thenb = self.statement();
        let mut elseb = None;
        // Avoid the dangling else by checking for it eagerly.
        if self.maybe_advance(vec![TokenType::Else]) {
            elseb = Some(self.statement());
        }
        return Stmt::If(condition, Box::new(thenb), Box::new(elseb));
    }

    fn expression(&mut self) -> Expr {
        //self.equality()
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.parse_or();

        if self.maybe_advance(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            match expr {
                Expr::Variable(tok) => {
                    return Expr::Assign(tok, Box::new(value));
                }
                _ => {
                    let emsg = format!("{} is an invalid assignment target", equals);
                    panic!("{}", emsg);
                }
            }
        }
        return expr;
    }

    /// parse logical ors
    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();

        while self.maybe_advance(vec![TokenType::Or]) {
            let op = self.previous();
            let right = self.parse_and();
            expr = Expr::Logic(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.maybe_advance(vec![TokenType::And]) {
            let op = self.previous();
            let right = self.equality();
            expr = Expr::Logic(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse equality expressions (== and !=)
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.maybe_advance(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse >, >=, < and <= expressions
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.maybe_advance(vec![
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
        while self.maybe_advance(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse * and / expressions
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.maybe_advance(vec![TokenType::Star, TokenType::Slash]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Unary expressions like ! which negates boolean or - which negates a number
    /// are parsed here
    fn unary(&mut self) -> Expr {
        // Check to see if it's a ! or -. If it is ten it's a unary expression
        // so grab the token and recurse un unary to parse te operand.
        if self.maybe_advance(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary(op, Box::new(right))
        } else {
            // If it's not a unary expr, then we have a primary expression
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.maybe_advance(vec![TokenType::False]) {
            return Expr::Literal(Literal::Bool(false));
        }
        if self.maybe_advance(vec![TokenType::True]) {
            return Expr::Literal(Literal::Bool(true));
        }
        if self.maybe_advance(vec![TokenType::None]) {
            return Expr::Literal(Literal::Empty);
        }

        if self.maybe_advance(vec![TokenType::Number]) {
            return Expr::Literal(self.previous().literal);
        }

        if self.maybe_advance(vec![TokenType::String]) {
            return Expr::Literal(self.previous().literal);
        }

        if self.maybe_advance(vec![TokenType::Identifier]) {
            return Expr::Variable(self.previous());
        }

        if self.maybe_advance(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen);
            return Expr::Grouping(Box::new(expr));
        }

        panic!("Bottomed out of Parser::primary with {}", self.peek());
    }

    // Helper functions that abstract out common logic.

    /// Consume the next token, if it matches the provided token variant.
    fn consume(&mut self, ttype: TokenType) -> Token {
        //if self.expect(vec![ttype.clone()]) {
        if self.check(ttype.clone()) {
            return self.next();
        } else {
            panic!(
                "Was expecting token {:?}, but found {:?} at token {} ({:?})",
                ttype,
                self.peek().ttype,
                self.current,
                self.peek()
            );
        }
    }

    /// Given a list of valid tokentypes, see if the next token in the stream is
    /// in that list, and thus valid
    fn maybe_advance(&mut self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.check(ttype) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Checks if the given token type is the next token type. Does NOT consume.
    fn check(&mut self, ttype: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        return self.peek().ttype == ttype;
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
