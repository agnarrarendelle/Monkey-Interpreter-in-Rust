use std::{fmt};


#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Hash, Ord)]
pub enum Token  {
    ILLEGAL,
    EOF,

    IDENT(String),

    INT(String),
    BOOLEAN(bool),
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
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,

    STRING(String)
}



pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::BOOLEAN(true),
        "false" => Token::BOOLEAN(false),
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(ident.to_string()),
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::IDENT(id) => write!(f, "{}", id),
            Token::INT(i) => write!(f, "{}", i),
            Token::BOOLEAN(b) => write!(f, "{}", b),
            Token::ASSIGN => write!(f, "="),
            Token::PLUS => write!(f, "+"),
            Token::MINUS => write!(f, "-"),
            Token::BANG => write!(f, "!"),
            Token::ASTERISK => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::EQ => write!(f, "=="),
            Token::NOTEQ => write!(f, "!="),
            Token::COMMA => write!(f, ","),
            Token::COLON => write!(f, ":"),
            Token::SEMICOLON => write!(f, ";"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::LBRACKET=>write!(f, "["),
            Token::RBRACKET=>write!(f, "]"),
            Token::FUNCTION => write!(f, "fn"),
            Token::LET => write!(f, "let"),
            Token::RETURN => write!(f, "return"),
            Token::IF => write!(f, "if"),
            Token::ELSE => write!(f, "else"),
            Token::EOF=>write!(f, "EOF"),
            Token::ILLEGAL => write!(f, "invalid token"),
            Token::STRING(s) => write!(f, "{}",s),
        }
    }
}

