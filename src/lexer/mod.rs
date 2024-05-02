
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
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::Assign,
            b'!' => Token::Band,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b',' => Token::Commas,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::EOF,
            _ => {
                if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    let name = self.read_identifier();
                    return Token::new_id(name);
                }
                if self.ch.is_ascii_digit() {
                    let value = self.read_number();
                    return Token::new_number(value);
                }
                Token::Illegal
            }
        };
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
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

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        &self.input[start .. self.position]
    }

    fn read_number(&mut self) -> &str {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'.' {
            self.read_char();
        }
        &self.input[start .. self.position]
    }
}