
use std::collections::HashMap;
use std::iter::Map;
use std::string::ParseError;

use crate::lexer::{token, Lexer};
use crate::lexer::token::*;
use crate::ast::{self, ExpressionStatement, NumberLiteral};

mod parser_test;

type PrefixParseFn = fn(& mut Parser) -> Box<dyn ast::Expression>;
type InfixParseFn = fn(Box<dyn ast::Expression>) -> Box<dyn ast::Expression>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Equals,         // ==
    LessGreater,    // > or <
    Sum,            // + or -
    Product,        // * or /
    Prefix,         // - or !
    Call,           // myFunction()
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    current_token: token::Token<'a>,
    peak_token: token::Token<'a>,

    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser{
            lexer: lexer,
            current_token: TokenKind::EOF.into(),
            peak_token: TokenKind::EOF.into(),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.next_token();
        parser.next_token();

        parser.prefix_parse_fns.insert(TokenKind::Identifier, Self::parse_identifier);
        parser.prefix_parse_fns.insert(TokenKind::Number, Self::parse_number);
        return parser;
    }

    pub fn errors(&self) -> &Vec<String> { &self.errors }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut statements = Vec::new();
        while self.peak_token.kind != TokenKind::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }
        return ast::Program{ statements };
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if !self.expect_peak(TokenKind::Identifier) {
            return None;
        }
        let id = ast::Identifier {
            name: self.current_token.literal.to_string()
        };
        if !self.expect_peak(TokenKind::Assign) {
            return None;
        }
        // TODO: parse assignee
        while self.current_token.kind != TokenKind::Semicolon {
            self.next_token();
        }
        let s = ast::LetStatement{
            id,
            value: Box::new(ast::NoOpExpression{})
        };
        return Some(Box::new(s));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        self.next_token();
        // TODO: parse expression
        while self.current_token.kind != TokenKind::Semicolon {
            self.next_token();
        }
        let s = ast::ReturnStatement{
            value: Box::new(ast::NoOpExpression{})
        };
        return Some(Box::new(s));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            return Some(
                Box::new(ExpressionStatement{ expression })
            );
        }
        return None;
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn ast::Expression>> {
        if let Some(prefix_fn) = self.prefix_parse_fns.get(&self.current_token.kind) {
            return Some(prefix_fn(self));
        }
        return None;
    }

    fn parse_identifier(parser: &mut Parser) -> Box<dyn ast::Expression> {
        return Box::new(ast::Identifier{ name: parser.current_token.literal.to_string() })
    }

    fn parse_number(parser: &mut Parser) -> Box<dyn ast::Expression> {
        let literal = parser.current_token.literal;
        if literal.starts_with("0x") || literal.starts_with("0X") {
            let value = i64::from_str_radix(&literal[2..], 16).unwrap() as f64;
            return Box::new(NumberLiteral{value})
        }
        if literal.starts_with('0') {
            let value = i64::from_str_radix(literal, 8).unwrap() as f64;
            return Box::new(NumberLiteral{value})
        }
        return Box::new(NumberLiteral{value: literal.parse().unwrap()});
    }

    fn next_token(&mut self) {
        self.current_token.clone_from(&self.peak_token);
        self.peak_token = self.lexer.next_token();
    }

    fn expect_peak(&mut self, expect: TokenKind) -> bool {
        if self.peak_token.kind == expect {
            self.next_token();
            true
        } else {
            self.peak_error(expect);
            false
        }
    }

    fn peak_error(&mut self, expect: TokenKind) {
        let msg = format!("expected next token to be {:?}, got {:?} instead",
                expect, self.peak_token.kind);
        self.errors.push(msg);
    }
}