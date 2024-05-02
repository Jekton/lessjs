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

#[test]
fn test_keywords() {
    let input = r"
function foo() {
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
}
    ";
    let expects = [
        Token::Function,
        Token::Identifier { name: "foo".to_string() },
        Token::LParen,
        Token::RParen,
        Token::LBrace,

        Token::If,
        Token::LParen,
        Token::Number { value: 5.0 },
        Token::LT,
        Token::Number { value: 10.0 },
        Token::RParen,

        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,

        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,

        Token::RBrace,
        Token::EOF,
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_function_call() {
    let input = r"
let sum = add(1, 2);
console.log(sum);
    ";
    let expects = [
        Token::Let,
        Token::Identifier { name: "sum".to_string() },
        Token::Assign,
        Token::Identifier { name: "add".to_string() },
        Token::LParen,
        Token::Number { value: 1.0 },
        Token::Commas,
        Token::Number { value: 2.0 },
        Token::RParen,
        Token::Semicolon,

        Token::Identifier { name: "console".to_string() },
        Token::Dot,
        Token::Identifier { name: "log".to_string() },
        Token::LParen,
        Token::Identifier { name: "sum".to_string() },
        Token::RParen,
        Token::Semicolon,
        Token::EOF,
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_eq_ne() {
    let input = r"
9 == 9;
9 != 10;
10 !== 11;
11 === 11;
    ";
    let expects = [
        Token::Number { value: 9.0 },
        Token::EQ,
        Token::Number { value: 9.0 },
        Token::Semicolon,

        Token::Number { value: 9.0 },
        Token::NE,
        Token::Number { value: 10.0 },
        Token::Semicolon,

        Token::Number { value: 10.0 },
        Token::SNE,
        Token::Number { value: 11.0 },
        Token::Semicolon,

        Token::Number { value: 11.0 },
        Token::SEQ,
        Token::Number { value: 11.0 },
        Token::Semicolon,
    ];
    test_lexer(input, &expects);
}