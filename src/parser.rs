use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokenize();
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }

    fn match_token(&mut self, token_types: &[Token]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            panic!("{} Got {:?}", message, self.peek());
        }
    }

    fn skip_newlines(&mut self) {
        while self.match_token(&[Token::Newline]) {}
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !self.is_at_end() {
            statements.push(self.statement());
            self.skip_newlines();
        }

        let mut program = Program::new();
        program.statements = statements;
        program
    }

    fn statement(&mut self) -> Statement {
        self.skip_newlines();

        match self.peek() {
            Token::Def => self.function_def(),
            Token::Class => self.class_def(),
            Token::Import => self.import_statement(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::For => self.for_statement(),
            Token::Return => self.return_statement(),
            Token::Break => {
                self.advance();
                self.skip_newlines();
                Statement::Break
            }
            Token::Continue => {
                self.advance();
                self.skip_newlines();
                Statement::Continue
            }
            Token::Pass => {
                self.advance();
                self.skip_newlines();
                Statement::Pass
            }
            Token::Identifier(_) => {
                let start_pos = self.current;
                let name = if let Token::Identifier(n) = self.advance() {
                    n
                } else {
                    unreachable!()
                };

                if self.match_token(&[Token::Colon]) {
                    let type_annotation = self.parse_type();
                    let initializer = if self.match_token(&[Token::Equal]) {
                        Some(self.expression())
                    } else {
                        None
                    };
                    self.skip_newlines();
                    Statement::VarDecl {
                        name,
                        type_annotation,
                        initializer,
                    }
                } else {
                    self.current = start_pos;
                    let expr = self.expression();
                    self.skip_newlines();
                    Statement::Expression(expr)
                }
            }
            _ => {
                let expr = self.expression();
                self.skip_newlines();
                Statement::Expression(expr)
            }
        }
    }

    fn import_statement(&mut self) -> Statement {
        self.consume(Token::Import, "Expected 'import'");

        let path = if let Token::StringLiteral(p) = self.advance() {
            p
        } else {
            panic!("Expected string literal after 'import'");
        };

        self.skip_newlines();
        Statement::Import { path }
    }

    fn function_def(&mut self) -> Statement {
        self.consume(Token::Def, "Expected 'def'");
        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            panic!("Expected function name");
        };

        self.consume(Token::LeftParen, "Expected '(' after function name");
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                let param_name = if let Token::Identifier(n) = self.advance() {
                    n
                } else {
                    panic!("Expected parameter name");
                };

                self.consume(Token::Colon, "Expected ':' after parameter name");
                let param_type = self.parse_type();

                params.push(Parameter {
                    name: param_name,
                    param_type,
                });

                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters");

        let return_type = if self.match_token(&[Token::Arrow]) {
            self.parse_type()
        } else {
            Type::Void
        };

        self.consume(Token::LeftBrace, "Expected '{' before function body");
        let body = self.block();
        self.consume(Token::RightBrace, "Expected '}' after function body");

        Statement::FunctionDef {
            name,
            params,
            return_type,
            body,
        }
    }

    fn class_def(&mut self) -> Statement {
        self.consume(Token::Class, "Expected 'class'");
        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            panic!("Expected class name");
        };

        let base_class = if self.match_token(&[Token::LeftParen]) {
            let base = if let Token::Identifier(n) = self.advance() {
                Some(n)
            } else {
                None
            };
            self.consume(Token::RightParen, "Expected ')' after base class");
            base
        } else {
            None
        };

        self.consume(Token::LeftBrace, "Expected '{' before class body");
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        self.skip_newlines();

        // Parse field declarations first
        while !self.check(&Token::RightBrace) && !self.is_at_end() && !self.check(&Token::Def) {
            // Field declaration: name: type
            if let Token::Identifier(field_name) = self.advance() {
                self.consume(Token::Colon, "Expected ':' after field name");
                let field_type = self.parse_type();
                fields.push(crate::ast::Field {
                    name: field_name,
                    field_type,
                });
                self.skip_newlines();
            } else {
                panic!("Expected field name in class body");
            }
        }

        // Parse method definitions
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            methods.push(self.function_def());
            self.skip_newlines();
        }

        self.consume(Token::RightBrace, "Expected '}' after class body");

        Statement::ClassDef {
            name,
            base_class,
            fields,
            methods,
        }
    }

    fn if_statement(&mut self) -> Statement {
        self.consume(Token::If, "Expected 'if'");
        let condition = self.expression();
        self.consume(Token::LeftBrace, "Expected '{' after if condition");
        let then_branch = self.block();
        self.consume(Token::RightBrace, "Expected '}' after if body");

        let mut elif_branches = Vec::new();
        while self.match_token(&[Token::Elif]) {
            let elif_condition = self.expression();
            self.consume(Token::LeftBrace, "Expected '{' after elif condition");
            let elif_body = self.block();
            self.consume(Token::RightBrace, "Expected '}' after elif body");
            elif_branches.push((elif_condition, elif_body));
        }

        let else_branch = if self.match_token(&[Token::Else]) {
            self.consume(Token::LeftBrace, "Expected '{' after else");
            let else_body = self.block();
            self.consume(Token::RightBrace, "Expected '}' after else body");
            Some(else_body)
        } else {
            None
        };

        Statement::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        }
    }

    fn while_statement(&mut self) -> Statement {
        self.consume(Token::While, "Expected 'while'");
        let condition = self.expression();
        self.consume(Token::LeftBrace, "Expected '{' after while condition");
        let body = self.block();
        self.consume(Token::RightBrace, "Expected '}' after while body");

        Statement::While { condition, body }
    }

    fn for_statement(&mut self) -> Statement {
        self.consume(Token::For, "Expected 'for'");
        let variable = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            panic!("Expected variable name in for loop");
        };

        self.consume(Token::In, "Expected 'in' in for loop");
        let iterable = self.expression();
        self.consume(Token::LeftBrace, "Expected '{' after for clause");
        let body = self.block();
        self.consume(Token::RightBrace, "Expected '}' after for body");

        Statement::For {
            variable,
            iterable,
            body,
        }
    }

    fn return_statement(&mut self) -> Statement {
        self.consume(Token::Return, "Expected 'return'");
        let value = if self.check(&Token::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.expression())
        };
        self.skip_newlines();
        Statement::Return(value)
    }

    fn block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.statement());
            self.skip_newlines();
        }

        statements
    }

    fn parse_type(&mut self) -> Type {
        let base_type = match self.peek() {
            Token::IntType => {
                self.advance();
                Type::Int
            }
            Token::FloatType => {
                self.advance();
                Type::Float
            }
            Token::BoolType => {
                self.advance();
                Type::Bool
            }
            Token::StrType => {
                self.advance();
                Type::Str
            }
            Token::ListType => {
                self.advance();
                self.consume(Token::LeftBracket, "Expected '[' after 'list'");
                let elem_type = Box::new(self.parse_type());
                self.consume(Token::RightBracket, "Expected ']' after list element type");
                return Type::List(elem_type);
            }
            Token::DictType => {
                self.advance();
                self.consume(Token::LeftBracket, "Expected '[' after 'dict'");
                let key_type = Box::new(self.parse_type());
                self.consume(Token::Comma, "Expected ',' after dict key type");
                let val_type = Box::new(self.parse_type());
                self.consume(Token::RightBracket, "Expected ']' after dict value type");
                return Type::Dict(key_type, val_type);
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Type::Custom(name)
            }
            _ => panic!("Expected type, got {:?}", self.peek()),
        };

        // Check for array type suffix: int[5]
        if self.match_token(&[Token::LeftBracket]) {
            if let Token::IntLiteral(size) = self.peek() {
                let size = *size as usize;
                self.advance();
                self.consume(Token::RightBracket, "Expected ']' after array size");
                return Type::Array(Box::new(base_type), size);
            } else {
                panic!("Expected integer literal for array size");
            }
        }

        base_type
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        let expr = self.or();

        if let Expression::Variable(name) = &expr {
            if self.match_token(&[Token::Equal]) {
                let value = Box::new(self.assignment());
                return Expression::Assignment {
                    target: name.clone(),
                    value,
                };
            }
        }

        expr
    }

    fn or(&mut self) -> Expression {
        let mut expr = self.and();

        while self.match_token(&[Token::Or]) {
            let right = Box::new(self.and());
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right,
            };
        }

        expr
    }

    fn and(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.match_token(&[Token::And]) {
            let right = Box::new(self.equality());
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right,
            };
        }

        expr
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(&[Token::DoubleEqual, Token::NotEqual]) {
            let op = match &self.tokens[self.current - 1] {
                Token::DoubleEqual => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.comparison());
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(&[Token::Less, Token::Greater, Token::LessEqual, Token::GreaterEqual]) {
            let op = match &self.tokens[self.current - 1] {
                Token::Less => BinaryOp::Less,
                Token::Greater => BinaryOp::Greater,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.term());
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match &self.tokens[self.current - 1] {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            let right = Box::new(self.factor());
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(&[Token::Star, Token::Slash, Token::Percent, Token::DoubleSlash]) {
            let op = match &self.tokens[self.current - 1] {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                Token::DoubleSlash => BinaryOp::FloorDivide,
                _ => unreachable!(),
            };
            let right = Box::new(self.unary());
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(&[Token::Not, Token::Minus]) {
            let op = match &self.tokens[self.current - 1] {
                Token::Not => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            let operand = Box::new(self.unary());
            return Expression::Unary { op, operand };
        }

        self.power()
    }

    fn power(&mut self) -> Expression {
        let mut expr = self.call();

        if self.match_token(&[Token::DoubleStar]) {
            let right = Box::new(self.unary());
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Power,
                right,
            };
        }

        expr
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary();

        loop {
            if self.match_token(&[Token::LeftParen]) {
                let mut args = Vec::new();
                if !self.check(&Token::RightParen) {
                    loop {
                        args.push(self.expression());
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(Token::RightParen, "Expected ')' after arguments");
                expr = Expression::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.match_token(&[Token::LeftBracket]) {
                let index = self.expression();
                self.consume(Token::RightBracket, "Expected ']' after index");
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&[Token::Dot]) {
                let member = if let Token::Identifier(n) = self.peek().clone() {
                    self.advance();
                    n
                } else {
                    panic!("Expected member name after '.'");
                };

                // Check if this is a method call
                if self.match_token(&[Token::LeftParen]) {
                    let mut args = Vec::new();
                    if !self.check(&Token::RightParen) {
                        loop {
                            args.push(self.expression());
                            if !self.match_token(&[Token::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RightParen, "Expected ')' after method arguments");
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: member,
                        args,
                    };
                } else {
                    expr = Expression::MemberAccess {
                        object: Box::new(expr),
                        member,
                    };
                }
            } else {
                break;
            }
        }

        expr
    }

    fn parse_fstring(&mut self, fstring: String) -> Expression {
        let mut parts = Vec::new();
        let mut expressions = Vec::new();
        let mut current_part = String::new();
        let mut chars = fstring.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check for escaped {{
                if chars.peek() == Some(&'{') {
                    current_part.push('{');
                    chars.next();
                    continue;
                }

                // Save current string part
                parts.push(current_part.clone());
                current_part.clear();

                // Parse expression inside {}
                let mut expr_str = String::new();
                let mut brace_depth = 1;
                while let Some(ch) = chars.next() {
                    if ch == '{' {
                        brace_depth += 1;
                        expr_str.push(ch);
                    } else if ch == '}' {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            break;
                        }
                        expr_str.push(ch);
                    } else {
                        expr_str.push(ch);
                    }
                }

                // Parse the expression
                let mut lexer = crate::lexer::Lexer::new(expr_str);
                let mut temp_parser = Parser::new(lexer);
                let expr = temp_parser.expression();
                expressions.push(expr);
            } else if ch == '}' {
                // Check for escaped }}
                if chars.peek() == Some(&'}') {
                    current_part.push('}');
                    chars.next();
                } else {
                    // Unmatched }
                    panic!("Unmatched '}}' in f-string");
                }
            } else {
                current_part.push(ch);
            }
        }

        // Add final part
        parts.push(current_part);

        Expression::FString { parts, expressions }
    }

    fn primary(&mut self) -> Expression {
        match self.peek().clone() {
            Token::IntLiteral(n) => {
                self.advance();
                Expression::IntLiteral(n)
            }
            Token::FloatLiteral(f) => {
                self.advance();
                Expression::FloatLiteral(f)
            }
            Token::StringLiteral(s) => {
                self.advance();
                Expression::StringLiteral(s)
            }
            Token::FStringLiteral(s) => {
                self.advance();
                self.parse_fstring(s)
            }
            Token::True => {
                self.advance();
                Expression::BoolLiteral(true)
            }
            Token::False => {
                self.advance();
                Expression::BoolLiteral(false)
            }
            Token::None => {
                self.advance();
                Expression::NoneLiteral
            }
            Token::Identifier(name) => {
                self.advance();
                Expression::Variable(name)
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(Token::RightParen, "Expected ')' after expression");
                expr
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.expression());
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }

                self.consume(Token::RightBracket, "Expected ']' after array/list elements");

                // For now, treat all [...] literals as list literals
                // The type checker will determine if they're valid arrays
                Expression::ListLiteral { elements }
            }
            Token::LeftBrace => {
                self.advance();
                let mut pairs = Vec::new();

                if !self.check(&Token::RightBrace) {
                    loop {
                        let key = self.expression();
                        self.consume(Token::Colon, "Expected ':' after dict key");
                        let value = self.expression();
                        pairs.push((key, value));

                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }

                self.consume(Token::RightBrace, "Expected '}' after dict pairs");
                Expression::DictLiteral { pairs }
            }
            _ => panic!("Unexpected token in expression: {:?}", self.peek()),
        }
    }
}
