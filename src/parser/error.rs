use std::fmt::{self};
use std::error::Error;

use crate::token;



#[derive(Debug,Clone)]
pub struct ParseError(String);

impl Error for ParseError{

}

impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl ParseError{
    pub fn new(err: String)->Self{
        Self(err)
    }

    pub fn parse_identifier_error(token: &token::Token)->Self{
        Self::new(format!("Expected an Identifier but got {}", token) )
    }
    pub fn parse_next_token_error(expect: &token::Token, actual:&token::Token)->Self{
        Self::new(format!("Expected token to be {} but got {}", expect, actual) )
    }
    pub fn parse_integer_error(num:&str)->Self{
        Self::new(format!("Cannot parse {} into integer", num) )
    }

    pub fn illegal_token_error()->Self{
        Self::new(format!("Cannot parse illegal token") )
    }
}