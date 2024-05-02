
#[derive(Debug, PartialEq)]
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

    Commas,     // ,
    Semicolon,  // ;

    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }

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