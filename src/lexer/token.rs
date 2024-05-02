
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EOF,

    Assign,     // =
    Plus,       // +
    Minus,      // -
    ASTERISK,   // *
    SLASH,      // /
    BAND,       // !

    LT,         // <
    GT,         // >

    Commas,     // ,
    Semicolon,  // ;

    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
}