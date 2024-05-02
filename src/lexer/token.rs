
#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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

    Identifier { name: String },
    Number { value: f64 },
}

impl Token {
    #[inline(always)]
    pub fn new_id(name: &str) -> Self {
        Token::Identifier { name: name.to_string() }
    }
    pub fn new_number(value: &str) -> Self {
        if value.starts_with("0x") || value.starts_with("0X") {
            return Token::Number {
                value: i64::from_str_radix(&value[2..], 16).unwrap() as f64
            }
        }
        if value.starts_with('0') {
            return Token::Number {
                value: i64::from_str_radix(value, 8).unwrap() as f64
            }
        }
        Token::Number { value: value.parse().unwrap() }
    }
}

lazy_static! {
    pub static ref KEYWORKS: HashMap<&'static str, Token> = {
        let mut map = HashMap::new();
        map.insert("if", Token::If);
        map.insert("else", Token::Else);
        map.insert("true", Token::True);
        map.insert("false", Token::False);
        map.insert("function", Token::Function);
        map.insert("let", Token::Let);
        map.insert("return", Token::Return);
        map
    };
}