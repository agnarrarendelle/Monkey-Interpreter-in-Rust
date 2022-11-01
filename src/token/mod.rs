
#[derive(Debug, PartialEq)]
pub enum Token  {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOTEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}



pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(ident.to_string()),
    }
}
