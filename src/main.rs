use std::io::{self, Write};

mod lexer;
mod repl;
mod token;

fn main() {
    println!("Feel free to type in commands");
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        if let Err(err) = repl::start(io::stdin(), io::stdout()){
            println!("fuck");
        }

    }
}
