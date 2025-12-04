use crate::ast::*;
use crate::lexer::{Lexer, SourceLocation, Token, TokenWithLocation};

pub struct Parser {
    tokens: Vec<TokenWithLocation>,
    current: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokenize();
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current].token
    }

    fn peek_location(&self) -> SourceLocation {
        self.tokens[self.current].location
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].token.clone()
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

    fn parse_error(&self, message: &str) -> ! {
        let location = self.peek_location();
        eprintln!("\n\x1b[31;1mParse Error:\x1b[0m {}", message);
        eprintln!("  \x1b[90mat {}\x1b[0m", location);
        eprintln!("  \x1b[90mgot: {:?}\x1b[0m", self.peek());
        std::process::exit(1);
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            self.parse_error(&format!("{} (expected {:?})", message, token));
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
            Token::Assert => {
                self.advance();
                let condition = self.expression();
                // Optional: parse message after comma
                let message = if self.match_token(&[Token::Comma]) {
                    if let Expression::StringLiteral(s) = self.expression() {
                        Some(s)
                    } else {
                        panic!("Assert message must be a string literal");
                    }
                } else {
                    None
                };
                self.skip_newlines();
                Statement::Assert { condition, message }
            }
            Token::Try => self.try_statement(),
            Token::Raise => self.raise_statement(),
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

                // Check for ++ or -- operators
                if self.match_token(&[Token::PlusPlus]) {
                    self.skip_newlines();
                    // Desugar x++ to x = x + 1
                    return Statement::Expression(Expression::Assignment {
                        target: name.clone(),
                        value: Box::new(Expression::Binary {
                            left: Box::new(Expression::Variable(name)),
                            op: BinaryOp::Add,
                            right: Box::new(Expression::IntLiteral(1)),
                        }),
                    });
                }
                if self.match_token(&[Token::MinusMinus]) {
                    self.skip_newlines();
                    // Desugar x-- to x = x - 1
                    return Statement::Expression(Expression::Assignment {
                        target: name.clone(),
                        value: Box::new(Expression::Binary {
                            left: Box::new(Expression::Variable(name)),
                            op: BinaryOp::Subtract,
                            right: Box::new(Expression::IntLiteral(1)),
                        }),
                    });
                }

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
            self.parse_error("Expected string literal after 'import'");
        };

        self.skip_newlines();
        Statement::Import { path }
    }

    fn function_def(&mut self) -> Statement {
        self.consume(Token::Def, "Expected 'def'");
        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            self.parse_error("Expected function name after 'def'");
        };

        self.consume(Token::LeftParen, "Expected '(' after function name");
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                let param_name = if let Token::Identifier(n) = self.advance() {
                    n
                } else {
                    self.parse_error("Expected parameter name in function definition");
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

        // Parse field declarations first (with optional decorators)
        while !self.check(&Token::RightBrace) && !self.is_at_end() && !self.check(&Token::Def) {
            // Collect decorators before field
            let mut decorators = Vec::new();
            while self.check(&Token::At) {
                decorators.push(self.parse_decorator());
                self.skip_newlines();
            }

            // Field declaration: name: type
            if let Token::Identifier(field_name) = self.advance() {
                self.consume(Token::Colon, "Expected ':' after field name");
                let field_type = self.parse_type();
                fields.push(crate::ast::Field {
                    name: field_name,
                    field_type,
                    decorators,
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
            _base_class: base_class,
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

    fn try_statement(&mut self) -> Statement {
        self.consume(Token::Try, "Expected 'try'");
        self.consume(Token::LeftBrace, "Expected '{' after try");
        let try_block = self.block();
        self.consume(Token::RightBrace, "Expected '}' after try body");

        let mut except_clauses = Vec::new();
        while self.match_token(&[Token::Except]) {
            // Parse exception type (optional)
            let exception_type = if let Token::Identifier(exc_type) = self.peek() {
                let exc = exc_type.clone();
                self.advance();
                Some(exc)
            } else {
                None // Catch all
            };

            // Parse "as var_name" (optional)
            let var_name = if self.match_token(&[Token::As]) {
                if let Token::Identifier(var) = self.advance() {
                    Some(var)
                } else {
                    panic!("Expected variable name after 'as'");
                }
            } else {
                None
            };

            self.consume(Token::LeftBrace, "Expected '{' after except clause");
            let body = self.block();
            self.consume(Token::RightBrace, "Expected '}' after except body");

            except_clauses.push(ExceptClause {
                exception_type,
                var_name,
                body,
            });
        }

        // Parse finally block (optional)
        let finally_block = if self.match_token(&[Token::Finally]) {
            self.consume(Token::LeftBrace, "Expected '{' after finally");
            let block = self.block();
            self.consume(Token::RightBrace, "Expected '}' after finally body");
            Some(block)
        } else {
            None
        };

        Statement::Try {
            try_block,
            except_clauses,
            finally_block,
        }
    }

    fn raise_statement(&mut self) -> Statement {
        let line = self.tokens[self.current].location.line;
        self.consume(Token::Raise, "Expected 'raise'");

        // Parse exception type (required)
        let exception_type = if let Token::Identifier(exc_type) = self.advance() {
            exc_type
        } else {
            panic!("Expected exception type after 'raise'");
        };

        // Parse message in parentheses
        self.consume(Token::LeftParen, "Expected '(' after exception type");
        let message = self.expression();
        self.consume(Token::RightParen, "Expected ')' after exception message");
        self.skip_newlines();

        Statement::Raise {
            exception_type,
            message,
            line,
        }
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

    /// Parse a decorator: @name or @name(key="value", ...)
    fn parse_decorator(&mut self) -> crate::ast::Decorator {
        self.consume(Token::At, "Expected '@'");

        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            panic!("Expected decorator name after '@'");
        };

        let mut args = std::collections::HashMap::new();

        // Check for optional arguments: @name(key="value", ...)
        if self.match_token(&[Token::LeftParen]) {
            // Parse named arguments
            if !self.check(&Token::RightParen) {
                loop {
                    // Parse: key="value"
                    let key = if let Token::Identifier(k) = self.advance() {
                        k
                    } else {
                        panic!("Expected argument name in decorator");
                    };

                    self.consume(Token::Equal, "Expected '=' after decorator argument name");

                    let value = if let Token::StringLiteral(v) = self.advance() {
                        v
                    } else {
                        panic!("Expected string value for decorator argument");
                    };

                    args.insert(key, value);

                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            self.consume(Token::RightParen, "Expected ')' after decorator arguments");
        }

        crate::ast::Decorator { name, args }
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
                Type::List(elem_type)
            }
            Token::DictType => {
                self.advance();
                self.consume(Token::LeftBracket, "Expected '[' after 'dict'");
                let key_type = Box::new(self.parse_type());
                self.consume(Token::Comma, "Expected ',' after dict key type");
                let val_type = Box::new(self.parse_type());
                self.consume(Token::RightBracket, "Expected ']' after dict value type");
                Type::Dict(key_type, val_type)
            }
            Token::Optional => {
                // Optional[T] syntax
                self.advance();
                self.consume(Token::LeftBracket, "Expected '[' after 'Optional'");
                let inner_type = Box::new(self.parse_type());
                self.consume(Token::RightBracket, "Expected ']' after Optional inner type");
                return Type::Optional(inner_type);
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

        // Check for nullable type suffix: str?
        if self.match_token(&[Token::Question]) {
            return Type::Optional(Box::new(base_type));
        }

        base_type
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        let expr = self.or();

        // Check for compound assignment operators
        if self.match_token(&[Token::PlusEqual, Token::MinusEqual, Token::StarEqual, Token::SlashEqual]) {
            let op_token = self.tokens[self.current - 1].token.clone();
            let right_value = Box::new(self.assignment());

            // Determine the binary operator
            let binary_op = match op_token {
                Token::PlusEqual => BinaryOp::Add,
                Token::MinusEqual => BinaryOp::Subtract,
                Token::StarEqual => BinaryOp::Multiply,
                Token::SlashEqual => BinaryOp::Divide,
                _ => unreachable!(),
            };

            // Desugar: x += 1 becomes x = x + 1
            if let Expression::Variable(name) = &expr {
                let new_value = Box::new(Expression::Binary {
                    left: Box::new(Expression::Variable(name.clone())),
                    op: binary_op,
                    right: right_value,
                });
                return Expression::Assignment {
                    target: name.clone(),
                    value: new_value,
                };
            }

            // For index assignments: arr[i] += 1 becomes arr[i] = arr[i] + 1
            if let Expression::Index { object, index, line } = expr {
                if let Expression::Variable(obj_name) = *object.clone() {
                    let new_value = Box::new(Expression::Binary {
                        left: Box::new(Expression::Index {
                            object: Box::new(Expression::Variable(obj_name.clone())),
                            index: index.clone(),
                            line,
                        }),
                        op: binary_op,
                        right: right_value,
                    });
                    return Expression::IndexAssignment {
                        object: obj_name,
                        index,
                        value: new_value,
                        line,
                    };
                }
            }

            panic!("Invalid compound assignment target");
        }

        if self.match_token(&[Token::Equal]) {
            let value = Box::new(self.assignment());

            // Check if this is a simple variable assignment
            if let Expression::Variable(name) = &expr {
                return Expression::Assignment {
                    target: name.clone(),
                    value,
                };
            }

            // Check if this is an index assignment (e.g., arr[0] = x or dict["key"] = x)
            if let Expression::Index { object, index, line } = expr {
                // Extract the object variable name
                if let Expression::Variable(obj_name) = *object {
                    return Expression::IndexAssignment {
                        object: obj_name,
                        index,
                        value,
                        line,
                    };
                }
            }

            panic!("Invalid assignment target");
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
            let op = match &self.tokens[self.current - 1].token {
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
            let op = match &self.tokens[self.current - 1].token {
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
            let op = match &self.tokens[self.current - 1].token {
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
            let op = match &self.tokens[self.current - 1].token {
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
            let op = match &self.tokens[self.current - 1].token {
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
                let line = self.tokens[self.current - 1].location.line; // Capture line of '('
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
                    line,
                };
            } else if self.match_token(&[Token::LeftBracket]) {
                let line = self.tokens[self.current - 1].location.line;
                let index = self.expression();
                self.consume(Token::RightBracket, "Expected ']' after index");
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    line,
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
                let lexer = crate::lexer::Lexer::new(expr_str);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_source(source: &str) -> Program {
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        parser.parse()
    }

    #[test]
    fn test_parse_variable_declaration() {
        let program = parse_source("x: int = 42");
        assert_eq!(program.statements.len(), 1);

        if let Statement::VarDecl { name, type_annotation, initializer } = &program.statements[0] {
            assert_eq!(name, "x");
            assert_eq!(*type_annotation, Type::Int);
            assert!(initializer.is_some());
        } else {
            panic!("Expected VarDecl statement");
        }
    }

    #[test]
    fn test_parse_function_definition() {
        let program = parse_source("def add(a: int, b: int) -> int { return a + b }");
        assert_eq!(program.statements.len(), 1);

        if let Statement::FunctionDef { name, params, return_type, body } = &program.statements[0] {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "a");
            assert_eq!(params[0].param_type, Type::Int);
            assert_eq!(params[1].name, "b");
            assert_eq!(params[1].param_type, Type::Int);
            assert_eq!(*return_type, Type::Int);
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected FunctionDef statement");
        }
    }

    #[test]
    fn test_parse_class_definition() {
        let source = r#"
class Person {
    name: str
    age: int

    def greet(self: Person) -> void {
        pass
    }
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::ClassDef { name, fields, methods, .. } = &program.statements[0] {
            assert_eq!(name, "Person");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "name");
            assert_eq!(fields[0].field_type, Type::Str);
            assert_eq!(fields[1].name, "age");
            assert_eq!(fields[1].field_type, Type::Int);
            assert_eq!(methods.len(), 1);
        } else {
            panic!("Expected ClassDef statement");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let program = parse_source("if x > 0 { y = 1 }");
        assert_eq!(program.statements.len(), 1);

        if let Statement::If { condition, then_branch, elif_branches, else_branch } = &program.statements[0] {
            assert!(matches!(condition, Expression::Binary { .. }));
            assert_eq!(then_branch.len(), 1);
            assert_eq!(elif_branches.len(), 0);
            assert!(else_branch.is_none());
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_parse_if_elif_else() {
        let source = r#"
if x > 10 {
    y = 1
} elif x > 5 {
    y = 2
} else {
    y = 3
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::If { elif_branches, else_branch, .. } = &program.statements[0] {
            assert_eq!(elif_branches.len(), 1);
            assert!(else_branch.is_some());
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let program = parse_source("while x < 10 { x = x + 1 }");
        assert_eq!(program.statements.len(), 1);

        if let Statement::While { condition, body } = &program.statements[0] {
            assert!(matches!(condition, Expression::Binary { .. }));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected While statement");
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let program = parse_source("for i in items { print_int(i) }");
        assert_eq!(program.statements.len(), 1);

        if let Statement::For { variable, iterable, body } = &program.statements[0] {
            assert_eq!(variable, "i");
            assert!(matches!(iterable, Expression::Variable(_)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected For statement");
        }
    }

    #[test]
    fn test_parse_break_continue() {
        let program = parse_source("while True { break }");
        if let Statement::While { body, .. } = &program.statements[0] {
            assert!(matches!(body[0], Statement::Break));
        } else {
            panic!("Expected While with Break");
        }

        let program = parse_source("while True { continue }");
        if let Statement::While { body, .. } = &program.statements[0] {
            assert!(matches!(body[0], Statement::Continue));
        } else {
            panic!("Expected While with Continue");
        }
    }

    #[test]
    fn test_parse_assert() {
        let program = parse_source("assert x == 5");
        assert_eq!(program.statements.len(), 1);

        if let Statement::Assert { condition, message } = &program.statements[0] {
            assert!(matches!(condition, Expression::Binary { .. }));
            assert!(message.is_none());
        } else {
            panic!("Expected Assert statement");
        }
    }

    #[test]
    fn test_parse_assert_with_message() {
        let program = parse_source(r#"assert x > 0, "x must be positive""#);

        if let Statement::Assert { message, .. } = &program.statements[0] {
            assert_eq!(message.as_ref().unwrap(), "x must be positive");
        } else {
            panic!("Expected Assert statement with message");
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let program = parse_source("def foo() -> int { return 42 }");

        if let Statement::FunctionDef { body, .. } = &program.statements[0] {
            if let Statement::Return(Some(expr)) = &body[0] {
                assert!(matches!(expr, Expression::IntLiteral(42)));
            } else {
                panic!("Expected Return statement");
            }
        } else {
            panic!("Expected FunctionDef");
        }
    }

    #[test]
    fn test_parse_binary_expressions() {
        let program = parse_source("x = 10 + 5 * 2");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            // Should parse as 10 + (5 * 2) due to precedence
            if let Expression::Binary { op, right, .. } = &**value {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(**right, Expression::Binary { op: BinaryOp::Multiply, .. }));
            } else {
                panic!("Expected Binary expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_comparison_operators() {
        let tests = vec![
            ("x == 5", BinaryOp::Equal),
            ("x != 5", BinaryOp::NotEqual),
            ("x < 5", BinaryOp::Less),
            ("x > 5", BinaryOp::Greater),
            ("x <= 5", BinaryOp::LessEqual),
            ("x >= 5", BinaryOp::GreaterEqual),
        ];

        for (source, expected_op) in tests {
            let program = parse_source(source);
            if let Statement::Expression(Expression::Binary { op, .. }) = &program.statements[0] {
                assert_eq!(*op, expected_op);
            } else {
                panic!("Expected Binary expression for {}", source);
            }
        }
    }

    #[test]
    fn test_parse_logical_operators() {
        let program = parse_source("x > 0 and y < 10");

        if let Statement::Expression(Expression::Binary { op, .. }) = &program.statements[0] {
            assert_eq!(*op, BinaryOp::And);
        } else {
            panic!("Expected And expression");
        }

        let program = parse_source("x == 0 or y == 0");

        if let Statement::Expression(Expression::Binary { op, .. }) = &program.statements[0] {
            assert_eq!(*op, BinaryOp::Or);
        } else {
            panic!("Expected Or expression");
        }
    }

    #[test]
    fn test_parse_unary_operators() {
        let program = parse_source("x = not True");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Unary { op, .. } = &**value {
                assert_eq!(*op, UnaryOp::Not);
            } else {
                panic!("Expected Unary expression");
            }
        } else {
            panic!("Expected Assignment");
        }

        let program = parse_source("x = -5");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Unary { op, .. } = &**value {
                assert_eq!(*op, UnaryOp::Negate);
            } else {
                panic!("Expected Unary expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_compound_assignment() {
        let tests = vec![
            ("x += 5", BinaryOp::Add),
            ("x -= 5", BinaryOp::Subtract),
            ("x *= 5", BinaryOp::Multiply),
            ("x /= 5", BinaryOp::Divide),
        ];

        for (source, expected_op) in tests {
            let program = parse_source(source);

            if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
                // Compound assignments are desugared to x = x op value
                if let Expression::Binary { op, .. } = &**value {
                    assert_eq!(*op, expected_op, "Failed for {}", source);
                } else {
                    panic!("Expected Binary expression in compound assignment for {}", source);
                }
            } else {
                panic!("Expected Assignment for {}", source);
            }
        }
    }

    #[test]
    fn test_parse_increment_decrement() {
        let program = parse_source("x++");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Binary { op, right, .. } = &**value {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(**right, Expression::IntLiteral(1)));
            } else {
                panic!("Expected Binary expression");
            }
        } else {
            panic!("Expected Assignment");
        }

        let program = parse_source("y--");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Binary { op, right, .. } = &**value {
                assert_eq!(*op, BinaryOp::Subtract);
                assert!(matches!(**right, Expression::IntLiteral(1)));
            } else {
                panic!("Expected Binary expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_list_literal() {
        let program = parse_source("x = [1, 2, 3, 4, 5]");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::ListLiteral { elements } = &**value {
                assert_eq!(elements.len(), 5);
            } else {
                panic!("Expected ListLiteral");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_dict_literal() {
        let program = parse_source(r#"x = {"a": 1, "b": 2}"#);

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::DictLiteral { pairs } = &**value {
                assert_eq!(pairs.len(), 2);
            } else {
                panic!("Expected DictLiteral");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_index_access() {
        let program = parse_source("x = arr[0]");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Index { object, index, .. } = &**value {
                assert!(matches!(**object, Expression::Variable(_)));
                assert!(matches!(**index, Expression::IntLiteral(0)));
            } else {
                panic!("Expected Index expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_index_assignment() {
        let program = parse_source("arr[0] = 42");

        if let Statement::Expression(Expression::IndexAssignment { object, index, value, .. }) = &program.statements[0] {
            assert_eq!(object, "arr");
            assert!(matches!(**index, Expression::IntLiteral(0)));
            assert!(matches!(**value, Expression::IntLiteral(42)));
        } else {
            panic!("Expected IndexAssignment");
        }
    }

    #[test]
    fn test_parse_function_call() {
        let program = parse_source("print_int(42)");

        if let Statement::Expression(Expression::Call { callee, args, line: _ }) = &program.statements[0] {
            assert!(matches!(**callee, Expression::Variable(_)));
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_method_call() {
        let program = parse_source("obj.method(1, 2)");

        if let Statement::Expression(Expression::MethodCall { object, method, args }) = &program.statements[0] {
            assert!(matches!(**object, Expression::Variable(_)));
            assert_eq!(method, "method");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected MethodCall expression");
        }
    }

    #[test]
    fn test_parse_member_access() {
        let program = parse_source("x = obj.field");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::MemberAccess { object, member } = &**value {
                assert!(matches!(**object, Expression::Variable(_)));
                assert_eq!(member, "field");
            } else {
                panic!("Expected MemberAccess expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_import() {
        let program = parse_source(r#"import "module""#);

        if let Statement::Import { path } = &program.statements[0] {
            assert_eq!(path, "module");
        } else {
            panic!("Expected Import statement");
        }
    }

    #[test]
    fn test_parse_types() {
        let program = parse_source("x: int = 0");
        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            assert_eq!(*type_annotation, Type::Int);
        }

        let program = parse_source("x: float = 0.0");
        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            assert_eq!(*type_annotation, Type::Float);
        }

        let program = parse_source("x: bool = True");
        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            assert_eq!(*type_annotation, Type::Bool);
        }

        let program = parse_source(r#"x: str = "hello""#);
        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            assert_eq!(*type_annotation, Type::Str);
        }
    }

    #[test]
    fn test_parse_list_type() {
        let program = parse_source("x: list[int] = []");

        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            if let Type::List(elem_type) = type_annotation {
                assert_eq!(**elem_type, Type::Int);
            } else {
                panic!("Expected List type");
            }
        } else {
            panic!("Expected VarDecl");
        }
    }

    #[test]
    fn test_parse_dict_type() {
        let program = parse_source("x: dict[str, int] = {}");

        if let Statement::VarDecl { type_annotation, .. } = &program.statements[0] {
            if let Type::Dict(key_type, val_type) = type_annotation {
                assert_eq!(**key_type, Type::Str);
                assert_eq!(**val_type, Type::Int);
            } else {
                panic!("Expected Dict type");
            }
        } else {
            panic!("Expected VarDecl");
        }
    }

    #[test]
    fn test_parse_fstring() {
        let program = parse_source(r#"x = f"Hello {name}""#);

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::FString { parts, expressions } = &**value {
                assert_eq!(parts.len(), 2); // "Hello " and ""
                assert_eq!(expressions.len(), 1);
            } else {
                panic!("Expected FString expression");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_power_operator() {
        let program = parse_source("x = 2 ** 3");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Binary { op, .. } = &**value {
                assert_eq!(*op, BinaryOp::Power);
            } else {
                panic!("Expected Binary expression with Power");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_modulo_operator() {
        let program = parse_source("x = 10 % 3");

        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::Binary { op, .. } = &**value {
                assert_eq!(*op, BinaryOp::Modulo);
            } else {
                panic!("Expected Binary expression with Modulo");
            }
        } else {
            panic!("Expected Assignment");
        }
    }

    #[test]
    fn test_parse_literals() {
        let program = parse_source("x = 42");
        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            assert!(matches!(**value, Expression::IntLiteral(42)));
        }

        let program = parse_source("x = 3.14");
        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            assert!(matches!(**value, Expression::FloatLiteral(_)));
        }

        let program = parse_source(r#"x = "hello""#);
        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            if let Expression::StringLiteral(s) = &**value {
                assert_eq!(s, "hello");
            } else {
                panic!("Expected StringLiteral");
            }
        }

        let program = parse_source("x = True");
        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            assert!(matches!(**value, Expression::BoolLiteral(true)));
        }

        let program = parse_source("x = False");
        if let Statement::Expression(Expression::Assignment { value, .. }) = &program.statements[0] {
            assert!(matches!(**value, Expression::BoolLiteral(false)));
        }
    }

    #[test]
    fn test_parse_raise_statement() {
        let program = parse_source(r#"raise ValueError("Test error")"#);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Raise { exception_type, message, line: _ } = &program.statements[0] {
            assert_eq!(exception_type, "ValueError");
            assert!(matches!(message, Expression::StringLiteral(_)));
        } else {
            panic!("Expected Raise statement");
        }
    }

    #[test]
    fn test_parse_try_except() {
        let source = r#"
try {
    x = 1
} except ValueError {
    y = 2
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Try { try_block, except_clauses, finally_block } = &program.statements[0] {
            assert_eq!(try_block.len(), 1);
            assert_eq!(except_clauses.len(), 1);
            assert_eq!(except_clauses[0].exception_type, Some("ValueError".to_string()));
            assert_eq!(except_clauses[0].var_name, None);
            assert_eq!(except_clauses[0].body.len(), 1);
            assert!(finally_block.is_none());
        } else {
            panic!("Expected Try statement");
        }
    }

    #[test]
    fn test_parse_try_except_with_variable() {
        let source = r#"
try {
    x = 1
} except ValueError as e {
    y = 2
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Try { try_block, except_clauses, finally_block } = &program.statements[0] {
            assert_eq!(try_block.len(), 1);
            assert_eq!(except_clauses.len(), 1);
            assert_eq!(except_clauses[0].exception_type, Some("ValueError".to_string()));
            assert_eq!(except_clauses[0].var_name, Some("e".to_string()));
            assert_eq!(except_clauses[0].body.len(), 1);
            assert!(finally_block.is_none());
        } else {
            panic!("Expected Try statement");
        }
    }

    #[test]
    fn test_parse_try_multiple_except() {
        let source = r#"
try {
    x = 1
} except ValueError {
    y = 2
} except KeyError {
    z = 3
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Try { try_block, except_clauses, finally_block } = &program.statements[0] {
            assert_eq!(try_block.len(), 1);
            assert_eq!(except_clauses.len(), 2);
            assert_eq!(except_clauses[0].exception_type, Some("ValueError".to_string()));
            assert_eq!(except_clauses[1].exception_type, Some("KeyError".to_string()));
            assert!(finally_block.is_none());
        } else {
            panic!("Expected Try statement");
        }
    }

    #[test]
    fn test_parse_try_finally() {
        let source = r#"
try {
    x = 1
} finally {
    y = 2
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Try { try_block, except_clauses, finally_block } = &program.statements[0] {
            assert_eq!(try_block.len(), 1);
            assert_eq!(except_clauses.len(), 0);
            assert!(finally_block.is_some());
            assert_eq!(finally_block.as_ref().unwrap().len(), 1);
        } else {
            panic!("Expected Try statement");
        }
    }

    #[test]
    fn test_parse_try_except_finally() {
        let source = r#"
try {
    x = 1
} except ValueError {
    y = 2
} finally {
    z = 3
}
"#;
        let program = parse_source(source);
        assert_eq!(program.statements.len(), 1);

        if let Statement::Try { try_block, except_clauses, finally_block } = &program.statements[0] {
            assert_eq!(try_block.len(), 1);
            assert_eq!(except_clauses.len(), 1);
            assert_eq!(except_clauses[0].exception_type, Some("ValueError".to_string()));
            assert!(finally_block.is_some());
            assert_eq!(finally_block.as_ref().unwrap().len(), 1);
        } else {
            panic!("Expected Try statement");
        }
    }
}
