use std::io::{self};

mod lexer;
mod repl;
mod token;
mod ast;
mod parser;
mod object;
mod evaluator;

fn main() {
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
