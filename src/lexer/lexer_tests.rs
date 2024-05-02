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
        Token::Asterisk,
        Token::Slash,
        Token::Band,

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

#[test]
fn test_id() {
    let input = r" foo _fo0 bar_f23 _23  ";
    let expects = [
        Token::new_id("foo"),
        Token::new_id("_fo0"),
        Token::new_id("bar_f23"),
        Token::new_id("_23"),
        Token::EOF,
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_decimal() {
    let input = "12 34 0 42";
    let expects = [
        Token::Number{ value: 12.0 },
        Token::Number{ value: 34.0 },
        Token::Number{ value: 0.0 },
        Token::Number{ value: 42.0 },
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_octal() {
    let input = "012 034 0 042";
    let expects = [
        Token::Number{ value: 10.0 },
        Token::Number{ value: 28.0 },
        Token::Number{ value: 0.0 },
        Token::Number{ value: 34.0 },
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_hex() {
    let input = "0x12 0xff 0Xfd 0x42";
    let expects = [
        Token::Number{ value: 18.0 },
        Token::Number{ value: 255.0 },
        Token::Number{ value: 253.0 },
        Token::Number{ value: 66.0 },
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_float() {
    let input = "12.2 2.4e8";
    let expects = [
        Token::Number{ value: 12.2 },
        Token::Number{ value: 2.4e8 },
    ];
    test_lexer(input, &expects);
}

#[test]
#[should_panic]
fn test_illegal_number() {
    let input = "0129";
    let expects = [
        Token::Number{ value: 10.0 },
    ];
    test_lexer(input, &expects);
}

#[test]
#[should_panic]
fn test_illegal_number2() {
    let input = "1.2.9";
    let expects = [
        Token::Number{ value: 1.2 },
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_assign() {
    let input = r"
foo = 42;
_fo0 = 24;
bar_f23 = 12;
_23 = 21;
    ";
    let expects = [
        Token::new_id("foo"),
        Token::Assign,
        Token::new_number("42"),
        Token::Semicolon,

        Token::new_id("_fo0"),
        Token::Assign,
        Token::new_number("24"),
        Token::Semicolon,

        Token::new_id("bar_f23"),
        Token::Assign,
        Token::new_number("12"),
        Token::Semicolon,

        Token::new_id("_23"),
        Token::Assign,
        Token::new_number("21"),
        Token::Semicolon,
    ];
    test_lexer(input, &expects);
}