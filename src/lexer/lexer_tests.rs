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
        TokenKind::Assign,
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Asterisk,
        TokenKind::Slash,
        TokenKind::Band,

        TokenKind::LT,
        TokenKind::GT,
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBrace,
        TokenKind::RBrace,

        TokenKind::Commas,
        TokenKind::Semicolon,

        TokenKind::EOF,
    ].map(| kind | Token::from(kind) );
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
        TokenKind::EOF.into(),
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_decimal() {
    let input = "12 34 0 42";
    let expects = [
        Token::new_number("12"),
        Token::new_number("34"),
        Token::new_number("0"),
        Token::new_number("42"),
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_octal() {
    let input = "012 034 0 042";
    let expects = [
        Token::new_number("012"),
        Token::new_number("034"),
        Token::new_number("0"),
        Token::new_number("042"),
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_hex() {
    let input = "0x12 0xff 0Xfd 0x42";
    let expects = [
        Token::new_number("0x12"),
        Token::new_number("0xff"),
        Token::new_number("0Xfd"),
        Token::new_number("0x42"),
    ];
    test_lexer(input, &expects);
}

#[test]
fn test_float() {
    let input = "12.2 2.4e8";
    let expects = [
        Token::new_number("12.2"),
        Token::new_number("2.4e8"),
    ];
    test_lexer(input, &expects);
}


#[test]
fn test_illegal_number2() {
    let input = "1.2.9";
    let expects = [
        Token::new_number("1.2.9"),
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
        TokenKind::Assign.into(),
        Token::new_number("42"),
        TokenKind::Semicolon.into(),

        Token::new_id("_fo0"),
        TokenKind::Assign.into(),
        Token::new_number("24"),
        TokenKind::Semicolon.into(),

        Token::new_id("bar_f23"),
        TokenKind::Assign.into(),
        Token::new_number("12"),
        TokenKind::Semicolon.into(),

        Token::new_id("_23"),
        TokenKind::Assign.into(),
        Token::new_number("21"),
        TokenKind::Semicolon.into(),
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
        TokenKind::Function.into(),
        Token::new_id("foo"),
        TokenKind::LParen.into(),
        TokenKind::RParen.into(),
        TokenKind::LBrace.into(),

        TokenKind::If.into(),
        TokenKind::LParen.into(),
        Token::new_number("5"),
        TokenKind::LT.into(),
        Token::new_number("10"),
        TokenKind::RParen.into(),

        TokenKind::LBrace.into(),
        TokenKind::Return.into(),
        TokenKind::True.into(),
        TokenKind::Semicolon.into(),
        TokenKind::RBrace.into(),

        TokenKind::Else.into(),
        TokenKind::LBrace.into(),
        TokenKind::Return.into(),
        TokenKind::False.into(),
        TokenKind::Semicolon.into(),
        TokenKind::RBrace.into(),

        TokenKind::RBrace.into(),
        TokenKind::EOF.into(),
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
        TokenKind::Let.into(),
        Token::new_id("sum"),
        TokenKind::Assign.into(),
        Token::new_id("add"),
        TokenKind::LParen.into(),
        Token::new_number("1"),
        TokenKind::Commas.into(),
        Token::new_number("2"),
        TokenKind::RParen.into(),
        TokenKind::Semicolon.into(),

        Token::new_id("console"),
        TokenKind::Dot.into(),
        Token::new_id("log"),
        TokenKind::LParen.into(),
        Token::new_id("sum"),
        TokenKind::RParen.into(),
        TokenKind::Semicolon.into(),
        TokenKind::EOF.into(),
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
        Token::new_number("9"),
        TokenKind::EQ.into(),
        Token::new_number("9"),
        TokenKind::Semicolon.into(),

        Token::new_number("9"),
        TokenKind::NE.into(),
        Token::new_number("10"),
        TokenKind::Semicolon.into(),

        Token::new_number("10"),
        TokenKind::SNE.into(),
        Token::new_number("11"),
        TokenKind::Semicolon.into(),

        Token::new_number("11"),
        TokenKind::SEQ.into(),
        Token::new_number("11"),
        TokenKind::Semicolon.into(),
    ];
    test_lexer(input, &expects);
}