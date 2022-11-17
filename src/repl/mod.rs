use std::{
    io::{stdout, BufRead, BufReader, Read, Write},
};

use crate::{lexer, parser};

pub fn start(input: impl Read, _output: impl Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();
    loop {
        print!(">> ");
        if let Err(e)=stdout().flush(){
            println!("Error: {:?}", e);
            break;
        }
        if let Err(e) = reader.read_line(&mut input) {
            println!("Error: {:?}", e);
            break;
        } else if input.trim() == "!quit" || input.trim() == "!q" {
            println!("exit");
            std::process::exit(0)
        } else {
            let  l = lexer::Lexer::new(&input);
            let mut p = parser::Parser::new(l);
            let program = p.parse_program();
            input.clear();
            match program {
                Err(errors) => {
                    for e in errors {
                        println!("{}", e)
                    }
                }
                Ok(p) => {
                    for stmt in p{
                        println!("{}", stmt)
                    }
                }
            }
        }
       
    }
   
}
