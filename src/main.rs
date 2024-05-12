use std::io::{stdin, stdout};

#[macro_use]
extern crate lazy_static;

mod lexer;
mod repl;

fn main() {
    println!("Hello, world!");
    repl::start(&mut stdin(), &mut stdout()).unwrap();
}
