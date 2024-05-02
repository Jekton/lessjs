#![cfg(test)]

use crate::lexer::token::*;
use super::Lexer;

fn test_lexer(input: &str, expects: &[Token]) {
    let mut lexer = Lexer::new(input);
    for expect in expects {
        let token = lexer.next_token();
        assert_eq!(&token, expect);
    }
}

#[test]
fn test_simple_token() {
    let input = "=+-*/!<>(){},;";
    let expects = [
        Token::Assign,
        Token::Plus,
        Token::Minus,
        Token::ASTERISK,
        Token::SLASH,
        Token::BAND,

        Token::LT,
        Token::GT,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,

        Token::Commas,
        Token::Semicolon,

        Token::EOF,
    ];
    test_lexer(input, &expects);
}
