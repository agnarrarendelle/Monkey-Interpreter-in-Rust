use crate::token::Token;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
	LESSGREATER, // > or <
	SUM,         // +
	PRODUCT,     // *
	PREFIX,      // -X or !X
	CALL        // myFunction(X)
}

pub fn token_to_precedence(token: &Token)->Precedence{
    match  token {
        Token::EQ | Token::NOTEQ=>Precedence::EQUALS,
        Token::GT | Token::LT=>Precedence::LESSGREATER,
        Token::PLUS | Token::MINUS=>Precedence::SUM,
        Token::SLASH | Token::ASTERISK=>Precedence::PRODUCT,
        _=>Precedence::LOWEST
    }
}