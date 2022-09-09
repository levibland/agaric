use std::{env, fs};
use crate::lexer::lex;

pub mod lexer;

fn main() {
    let path = env::args().nth(1).unwrap();

    let source = fs::read_to_string(path).unwrap();

    let tokens = lex(&source);

    println!("{:#?}", tokens);
}
