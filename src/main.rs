use std::io::{self};

mod lexer;
mod repl;
mod token;
mod ast;
mod parser;

fn main() {
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
