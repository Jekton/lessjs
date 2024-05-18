
#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Illegal,
    EOF,

    Assign,     // =
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /
    Band,       // !

    LT,         // <
    GT,         // >
    EQ,         // ==
    NE,         // !=
    SEQ,         // ===
    SNE,         // !==

    Commas,     // ,
    Semicolon,  // ;
    Dot,        // .

    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }

    If,
    Else,
    True,
    False,
    Function,
    Let,
    Return,

    Identifier,
    Number,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    #[inline(always)]
    pub fn new<'x: 'a>(kind: TokenKind, literal: &'x str) -> Self {
        return Token{ kind, literal }
    }

    #[inline(always)]
    pub fn new_id<'x: 'a>(id: &'x str) -> Self {
        return Token{ kind: TokenKind::Identifier, literal: id }
    }

    #[inline(always)]
    pub fn new_number<'x: 'a>(literal: &'x str) -> Self {
        return Token{ kind: TokenKind::Number, literal }
    }
}

impl<'a> From<TokenKind> for Token<'a>  {

    #[inline(always)]
    fn from(kind: TokenKind) -> Self {
        debug_assert_ne!(kind, TokenKind::Identifier);
        debug_assert_ne!(kind, TokenKind::Number);
        return Token { kind, literal: "" }
    }
}



lazy_static! {
    pub static ref KEYWORKS: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("if", TokenKind::If);
        map.insert("else", TokenKind::Else);
        map.insert("true", TokenKind::True);
        map.insert("false", TokenKind::False);
        map.insert("function", TokenKind::Function);
        map.insert("let", TokenKind::Let);
        map.insert("return", TokenKind::Return);
        map
    };
}