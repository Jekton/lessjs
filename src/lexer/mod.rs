
mod lexer_tests;
mod token;

use token::*;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut lexer = Lexer{
            input: input,
            position: 0,
            read_position: 0,
            ch: b'0',
        };
        lexer.read_char();
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'=' => Token::Assign,
            b'!' => Token::BAND,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b',' => Token::Commas,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::EOF,
            _ => Token::Illegal,
        };
        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}