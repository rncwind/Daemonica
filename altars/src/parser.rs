//! A Hand-written recurisve descent parser for Daemonica.
use crate::{
    ast::{ASTNode, Expr, Stmt},
    literals::Literal,
    token::Token,
    tokentype::TokenType,
};

/// The documentation for this crate is designed to follow the flow of the
/// recursive descent parser. As such each level of precidence and it's productions
/// are doccumented before each lower level of precidence.
pub struct Parser {
    /// Store the sequence of tokens that we recieve from the lexer, so we can
    /// recursivley parse them.
    tokens: Vec<Token>,
    /// Store a counter that indexes to the current token.
    current: usize,
}

impl Parser {
    /// Initialize a parser instance with some input set of tokens.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    /// Given an initialized Parser; produces a series of AST Nodes to be interpreted.
    ///
    /// # Example
    /// ```
    /// let mut s: Scanner = Scanner::new(src);
    /// let tokens = s.scan_tokens();
    /// let mut p: Parser = Parser::new(tokens);
    /// let AST = p.parse();
    /// ```
    pub fn parse_self(&mut self) -> Vec<ASTNode> {
        let mut stmts: Vec<ASTNode> = Vec::new();
        while self.at_end() == false {
            //stmts.push(ASTNode::StmtNode(self.statement()));
            stmts.push(ASTNode::StmtNode(self.declaration()));
        }
        return stmts;
    }

    /// Helper function that allows you to parse directly if you don't want
    /// to actually keep an instance of the parser.
    ///
    /// # Example
    /// ```
    /// let mut s: Scanner = scanner::new(src);
    /// let tokens = s.scan_tokens();
    /// let AST = Parser::parse_direct(tokens);
    /// ```
    pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
        let mut p = self::Parser::new(tokens);
        p.parse_self()
    }

    /// Top level of our parse tree. Checks for function definitions, or variable
    /// declarations first. If neither descends into statement.
    fn declaration(&mut self) -> Stmt {
        if self.maybe_advance(vec![TokenType::Fn]) {
            return self.function();
        }
        if self.maybe_advance(vec![TokenType::Var]) {
            return self.var_decl();
        }
        return self.statement();
    }

    /// Grab the function name, and the actual body of the function.
    fn function(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier);
        self.consume(TokenType::LeftParen);
        let mut params: Vec<Token> = Vec::new();
        if self.check(TokenType::RightParen) == false {
            loop {
                params.push(self.consume(TokenType::Identifier));
                if self.maybe_advance(vec![TokenType::Comma]) == false {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen);
        self.consume(TokenType::LeftBrace);
        let body = self.block();
        Stmt::Function(name, params, body)
    }

    /// Collect the variable name, then we need to see if we have an initializer
    /// statement or not. If we do we need to parse it as an expression and store
    /// it.
    /// If we dont we can simply store it as an [Option::None]
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

    /// 2nd Level of the parser.
    ///
    /// This level has multiple productions of the same precidence.
    /// Print and Return statements are both "detected" here, as are
    /// flow control statements like While, If and For.
    /// Block Statements are also parsed here.
    fn statement(&mut self) -> Stmt {
        if self.maybe_advance(vec![TokenType::Print]) {
            return self.parse_print();
        }
        if self.maybe_advance(vec![TokenType::Return]) {
            return self.parse_return();
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
            return self.parse_for();
        }
        return self.expression_stmt();
    }

    /// Take the right hand side as an expression and dump it wholesale
    /// into a new AST node.
    fn parse_print(&mut self) -> Stmt {
        let val = self.expression();
        self.consume(TokenType::Semicolon);
        return Stmt::Print(val);
    }

    /// Grab the right hand side of the expression, and throw it into the AST.
    /// The RHS in the AST node is an Option so that empty returns are allowed.
    ///
    /// ***NOTE: Return DOES NOT return control flow, merley a value.***
    fn parse_return(&mut self) -> Stmt {
        let prev = self.previous();
        let mut value = None;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression());
        }
        self.consume(TokenType::Semicolon);
        Stmt::Return(prev, value)
    }

    /// This produces a vector of statements, that we then wrap inside a [crate::ast::Stmt::Block].
    fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.at_end() {
            stmts.push(self.declaration());
        }
        self.consume(TokenType::RightBrace);
        stmts
    }

    /// Standard while loop construct. We grab the condition and the body
    /// of the while statement and produce a new AST Node.
    fn while_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen);
        let condition = self.expression();
        self.consume(TokenType::RightParen);
        let body = self.statement();
        return Stmt::While(condition, Box::new(body));
    }

    /// This handles both if and else constructs.
    /// Grab the condition and the body of the if statement.
    /// If an Else statement exists, when we parse that too; otherwise it's a None.
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

    /// We desugar for statments into their component parts.
    ///
    /// As any for loop
    /// can also be expressed as a while loop, with some extra code that runs
    /// before execution (initializer), and some code that runs
    /// before each iteration (incrementation etc).
    ///
    /// As such we can; instead of implementing
    /// the for loop itself; break the expression into it's components right now
    /// avoiding adding an explicit case to the interpreter, and another variant in our AST.
    fn parse_for(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen);
        // Parse out the various parts of our for statement, for desugaring
        // in a second

        // Parse our initializer
        let initializer: Option<Stmt>;
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

        // And finally get the body of the statement.
        let mut body = self.statement();

        match increment {
            // If we have an increment, then create the AST Node for it.
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

    /// In order to keep our grammar somewhat sane, we have the ability to wrap
    /// an expression inside a statement. This allows it to be a fair bit more
    /// flexable.
    fn expression_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon);
        Stmt::Expression(expr)
    }

    /// Top level for expressions.
    /// Recurses down to assignment
    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    /// Parse varaible assignment
    fn assignment(&mut self) -> Expr {
        let expr = self.parse_or();

        if self.maybe_advance(vec![TokenType::Equal]) {
            // If we have an assignment, then grab the value
            let equals = self.previous();
            let value = self.assignment();

            // If our expression that we recursed at the top is a variable, then
            // we bind the value we just got, to that name.
            match expr {
                Expr::Variable(tok) => {
                    return Expr::Assign(tok, Box::new(value));
                }
                // If not, we forgot to actually give the assignment it's RHS.
                _ => {
                    let emsg = format!("{} is an invalid assignment target", equals);
                    panic!("{}", emsg);
                }
            }
        }
        return expr;
    }

    /// parse logical ors
    ///
    /// Quite simple, grab the RHS and the LHS, throw them in a node.
    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();

        while self.maybe_advance(vec![TokenType::Or]) {
            let op = self.previous();
            let right = self.parse_and();
            expr = Expr::Logic(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    /// Parse logical And
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
            // If it's not a unary expr, then we might have a call
            self.call()
            //self.primary()
        }
    }

    /// This handles function calls.
    /// Extracts the name of the function, and (possibly) arguments
    /// and produces a new node with them.
    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.maybe_advance(vec![TokenType::LeftParen]) {
                expr = self.parse_arglist(expr);
            } else {
                break;
            }
        }
        return expr;
    }

    fn parse_arglist(&mut self, callee: Expr) -> Expr {
        let mut args: Vec<Expr> = Vec::new();
        if self.check(TokenType::RightParen) == false {
            loop {
                args.push(self.expression());
                if !self.maybe_advance(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen);
        Expr::Call(Box::new(callee), paren, args)
    }

    /// Bottom case is primary expressions.
    /// This includes all literals, variables and paren groupings.
    /// If we reach here, and nothing is valid then we've bottomed out
    /// the parser, and need to report an error to the user.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;

    #[test]
    fn vardecl() {
        let test: String = String::from("ligamen a = 5.3;");
        let tok = Scanner::scan(test);
        let result = Parser::parse(tok);
        let expected = vec![ASTNode::StmtNode(Stmt::Var(
            Token::new(TokenType::Identifier, String::from("a"), Literal::Empty, 1),
            Some(Expr::Literal(Literal::Number(5.3))),
        ))];
        assert!(result == expected);
    }

    #[test]
    fn vardel_no_initializer() {
        let test = String::from("ligamen a;");
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Var(
            Token::new(TokenType::Identifier, String::from("a"), Literal::Empty, 1),
            None,
        ))];
        assert!(res == expected);
    }

    #[test]
    fn fundecl() {
        let test: String = String::from(
            r#"
        incantatio fun {
          "Hi";
        }"#,
        );
        let res = process(test);

        let expected = vec![ASTNode::StmtNode(Stmt::Function(
            Token::new(
                TokenType::Identifier,
                String::from("fun"),
                Literal::Empty,
                2,
            ),
            vec![Stmt::Expression(Expr::Literal(Literal::StrLit(
                String::from("Hi"),
            )))],
        ))];
        assert!(res == expected);
    }

    #[test]
    fn printstmt() {
        let test: String = String::from("scribo 5.0;");
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Print(Expr::Literal(
            Literal::Number(5.0),
        )))];
        assert!(res == expected);
    }

    #[test]
    fn return_test() {
        let test = String::from(
            r#"
        incantatio fun {
          beneficium 5;
        }"#,
        );
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Function(
            Token::new(
                TokenType::Identifier,
                String::from("fun"),
                Literal::Empty,
                2,
            ),
            vec![Stmt::Return(
                Token::new(
                    TokenType::Return,
                    String::from("beneficium"),
                    Literal::Empty,
                    3,
                ),
                Some(Expr::Literal(Literal::Number(5.0))),
            )],
        ))];
        assert!(expected == res);
    }

    #[test]
    fn return_nothing() {
        let test = String::from(
            r#"
        incantatio fun {
          beneficium;
        }"#,
        );
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Function(
            Token::new(
                TokenType::Identifier,
                String::from("fun"),
                Literal::Empty,
                2,
            ),
            vec![Stmt::Return(
                Token::new(
                    TokenType::Return,
                    String::from("beneficium"),
                    Literal::Empty,
                    3,
                ),
                None,
            )],
        ))];
        assert!(expected == res);
    }

    #[test]
    fn blocks() {
        let test = String::from(
            r#"
            "Outside";
            {
                "Inside";
            }"#,
        );
        let res = process(test);
        let expected = vec![
            ASTNode::StmtNode(Stmt::Expression(Expr::Literal(Literal::StrLit(
                String::from("Outside"),
            )))),
            ASTNode::StmtNode(Stmt::Block(vec![Stmt::Expression(Expr::Literal(
                Literal::StrLit(String::from("Inside")),
            ))])),
        ];
        assert!(res == expected);
    }

    #[test]
    fn ifstmt() {
        let test = String::from(
            r#"
            si(1 == 2) {
                scribo "Maths is hard";
            }
            "#,
        );
        let expected = vec![ASTNode::StmtNode(Stmt::If(
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(1.0))),
                Token::new(TokenType::EqualEqual, String::from("=="), Literal::Empty, 2),
                Box::new(Expr::Literal(Literal::Number(2.0))),
            ),
            Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal(
                Literal::StrLit(String::from("Maths is hard")),
            ))])),
            Box::new(None),
        ))];
        let res = process(test);
        assert!(expected == res);
    }

    #[test]
    fn grouping() {
        let test = String::from("5 + (20 + 2 * (3));");
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Expression(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            Token {
                ttype: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: Literal::Empty,
                line: 1,
            },
            Box::new(Expr::Grouping(Box::new(Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(20.0))),
                Token {
                    ttype: TokenType::Plus,
                    lexeme: "+".to_string(),
                    literal: Literal::Empty,
                    line: 1,
                },
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(2.0))),
                    Token {
                        ttype: TokenType::Star,
                        lexeme: "*".to_string(),
                        literal: Literal::Empty,
                        line: 1,
                    },
                    Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
                        3.0,
                    ))))),
                )),
            )))),
        )))];
        assert!(expected == res);
    }

    #[test]
    fn unary() {
        let test = String::from("-5.0;");
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Expression(Expr::Unary(
            Token::new(TokenType::Minus, String::from("-"), Literal::Empty, 1),
            Box::new(Expr::Literal(Literal::Number(5.0))),
        )))];
        assert!(expected == res);

        let test = String::from("!verum;");
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Expression(Expr::Unary(
            Token::new(TokenType::Bang, String::from("!"), Literal::Empty, 1),
            Box::new(Expr::Literal(Literal::Bool(true))),
        )))];
        assert!(res == expected);
    }

    #[test]
    fn ifelse() {
        let test = String::from(
            r#"
            si(1 == 2) {
                scribo "Maths is hard";
            } aliter {
                scribo "Maths is easy";
            }
            "#,
        );
        let expected = vec![ASTNode::StmtNode(Stmt::If(
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(1.0))),
                Token::new(TokenType::EqualEqual, String::from("=="), Literal::Empty, 2),
                Box::new(Expr::Literal(Literal::Number(2.0))),
            ),
            Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal(
                Literal::StrLit(String::from("Maths is hard")),
            ))])),
            Box::new(Some(Stmt::Block(vec![Stmt::Print(Expr::Literal(
                Literal::StrLit(String::from("Maths is easy")),
            ))]))),
        ))];
        let res = process(test);
        assert!(expected == res);
    }

    #[test]
    fn forstmt() {
        let test = String::from(
            r#"
            enim(ligamen i = 1; i < 100; i = i + 1) {
                scribo i;
            }
            "#,
        );
        let res = process(test);
        let expected = vec![ASTNode::StmtNode(Stmt::Block(vec![
            Stmt::Var(
                Token {
                    ttype: TokenType::Identifier,
                    lexeme: "i".to_string(),
                    literal: Literal::Empty,
                    line: 2,
                },
                Some(Expr::Literal(Literal::Number(1.0))),
            ),
            Stmt::While(
                Expr::Binary(
                    Box::new(Expr::Variable(Token {
                        ttype: TokenType::Identifier,
                        lexeme: "i".to_string(),
                        literal: Literal::Empty,
                        line: 2,
                    })),
                    Token {
                        ttype: TokenType::Less,
                        lexeme: "<".to_string(),
                        literal: Literal::Empty,
                        line: 2,
                    },
                    Box::new(Expr::Literal(Literal::Number(100.0))),
                ),
                Box::new(Stmt::Block(vec![
                    Stmt::Block(vec![Stmt::Print(Expr::Variable(Token {
                        ttype: TokenType::Identifier,
                        lexeme: "i".to_string(),
                        literal: Literal::Empty,
                        line: 3,
                    }))]),
                    Stmt::Expression(Expr::Assign(
                        Token {
                            ttype: TokenType::Identifier,
                            lexeme: "i".to_string(),
                            literal: Literal::Empty,
                            line: 2,
                        },
                        Box::new(Expr::Binary(
                            Box::new(Expr::Variable(Token {
                                ttype: TokenType::Identifier,
                                lexeme: "i".to_string(),
                                literal: Literal::Empty,
                                line: 2,
                            })),
                            Token {
                                ttype: TokenType::Plus,
                                lexeme: "+".to_string(),
                                literal: Literal::Empty,
                                line: 2,
                            },
                            Box::new(Expr::Literal(Literal::Number(1.0))),
                        )),
                    )),
                ])),
            ),
        ]))];
        assert!(res == expected);
    }

    #[test]
    fn whilestmt() {
        let test = String::from(
            r#"
            ligamen a = 0;
            dum(a < 100) {
                a = a + 1;
            }
            "#,
        );
        let res = process(test);
        let expected = vec![
            ASTNode::StmtNode(Stmt::Var(
                Token {
                    ttype: TokenType::Identifier,
                    lexeme: String::from("a"),
                    literal: Literal::Empty,
                    line: 2,
                },
                Some(Expr::Literal(Literal::Number(0.0))),
            )),
            ASTNode::StmtNode(Stmt::While(
                Expr::Binary(
                    Box::new(Expr::Variable(Token {
                        ttype: TokenType::Identifier,
                        lexeme: String::from("a"),
                        literal: Literal::Empty,
                        line: 3,
                    })),
                    Token {
                        ttype: TokenType::Less,
                        lexeme: String::from("<"),
                        literal: Literal::Empty,
                        line: 3,
                    },
                    Box::new(Expr::Literal(Literal::Number(100.0))),
                ),
                Box::new(Stmt::Block(vec![Stmt::Expression(Expr::Assign(
                    Token {
                        ttype: TokenType::Identifier,
                        lexeme: String::from("a"),
                        literal: Literal::Empty,
                        line: 4,
                    },
                    Box::new(Expr::Binary(
                        Box::new(Expr::Variable(Token {
                            ttype: TokenType::Identifier,
                            lexeme: String::from("a"),
                            literal: Literal::Empty,
                            line: 4,
                        })),
                        Token {
                            ttype: TokenType::Plus,
                            lexeme: String::from("+"),
                            literal: Literal::Empty,
                            line: 4,
                        },
                        Box::new(Expr::Literal(Literal::Number(1.0))),
                    )),
                ))])),
            )),
        ];
        assert!(res == expected);
    }

    #[test]
    fn logic_ops() {
        let and = String::from("verum et nihil;");
        let andres = process(and);
        let expectedand = vec![ASTNode::StmtNode(Stmt::Expression(Expr::Logic(
            Box::new(Expr::Literal(Literal::Bool(true))),
            Token::new(TokenType::And, String::from("et"), Literal::Empty, 1),
            Box::new(Expr::Literal(Literal::Empty)),
        )))];
        assert!(andres == expectedand);

        let or = String::from("verum vel nihil;");
        let orres = process(or);

        let expectedor = vec![ASTNode::StmtNode(Stmt::Expression(Expr::Logic(
            Box::new(Expr::Literal(Literal::Bool(true))),
            Token::new(TokenType::Or, String::from("vel"), Literal::Empty, 1),
            Box::new(Expr::Literal(Literal::Empty)),
        )))];
        assert!(orres == expectedor);
    }

    fn process(testcase: String) -> Vec<ASTNode> {
        let tok = Scanner::scan(testcase);
        Parser::parse(tok)
    }
}
