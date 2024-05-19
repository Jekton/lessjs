
use crate::lexer::{token, Lexer};
use crate::lexer::token::*;
use crate::ast;

mod parser_test;

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    current_token: token::Token<'a>,
    peak_token: token::Token<'a>,

    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser{
            lexer: lexer,
            current_token: TokenKind::EOF.into(),
            peak_token: TokenKind::EOF.into(),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
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
            _ => None,
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