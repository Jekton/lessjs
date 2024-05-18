
mod lexer_tests;
pub mod token;

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
            b'=' => {
                if self.peak_char() == b'=' {
                    if self.peak_third_char() == b'=' {
                        self.read_char();
                        self.read_char();
                        Token::from(TokenKind::SEQ)
                    } else {
                        self.read_char();
                        Token::from(TokenKind::EQ)
                    }
                } else {
                    Token::from(TokenKind::Assign)
                }
            }
            b'!' => {
                if self.peak_char() == b'=' {
                    if self.peak_third_char() == b'=' {
                        self.read_char();
                        self.read_char();
                        Token::from(TokenKind::SNE)
                    } else {
                        self.read_char();
                        Token::from(TokenKind::NE)
                    }
                } else {
                    Token::from(TokenKind::Band)
                }
            }
            b'+' => Token::from(TokenKind::Plus),
            b'-' => Token::from(TokenKind::Minus),
            b'*' => Token::from(TokenKind::Asterisk),
            b'/' => Token::from(TokenKind::Slash),
            b'<' => Token::from(TokenKind::LT),
            b'>' => Token::from(TokenKind::GT),
            b',' => Token::from(TokenKind::Commas),
            b';' => Token::from(TokenKind::Semicolon),
            b'(' => Token::from(TokenKind::LParen),
            b')' => Token::from(TokenKind::RParen),
            b'{' => Token::from(TokenKind::LBrace),
            b'}' => Token::from(TokenKind::RBrace),
            b'.' => Token::from(TokenKind::Dot),
            0 => Token::from(TokenKind::EOF),
            _ => {
                if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    let name = self.read_identifier();
                    return Lexer::lookup_ident(name);
                }
                if self.ch.is_ascii_digit() {
                    let value = self.read_number();
                    return Token::new(TokenKind::Number, value)
                }
                Token::new(TokenKind::Illegal, "")
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

    fn peak_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn peak_third_char(&self) -> u8 {
        let pos = self.read_position + 1;
        if pos >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[pos]
        }
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

    fn lookup_ident(name: &str) -> Token {
        if let Some(kind) = KEYWORKS.get(name) {
            (*kind).into()
        } else {
            Token::new(TokenKind::Identifier, name)
        }
    }
}