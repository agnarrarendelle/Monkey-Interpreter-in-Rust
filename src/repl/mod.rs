use std::io::{BufRead, BufReader, Read, Write};

use crate::{lexer, token};

pub fn start(input: impl Read, output: impl Write) -> std::io::Result<()> {
    let reader = BufReader::new(input);

    let lines = reader.lines();
    for line in lines{
        let line = line?;

       
        let mut l = lexer::Lexer::new(&line);
        loop {
            let tok = l.next_token();
            if tok.token_type == token::Tokens::EOF {
                // break;
                return  Ok(());
            }
            println!("Literal:{},Type: {}", tok.literal, tok.token_type);
        }
        
    }
    Ok(())

}
