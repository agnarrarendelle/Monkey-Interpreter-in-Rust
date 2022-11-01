pub type TokenType = String;
pub struct Tokens;

impl Tokens {
    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";

    pub const IDENT: &str = "IDENT";
    pub const INT: &str = "INT";

    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const BANG: &str = "!";
    pub const ASTERISK: &str = "*";
    pub const SLASH: &str = "/";

    pub const LT: &str = "<";
    pub const GT: &str = ">";

    pub const EQ: &str = "==";
    pub const NOT_EQ: &str = "!=";

    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";

    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";

    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
    pub const TRUE: &str = "TRUE";
    pub const FALSE: &str = "FALSE";
    pub const IF: &str = "IF";
    pub const ELSE: &str = "ELSE";
    pub const RETURN: &str = "RETURN";
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

fn keyword_to_token(s: &str) -> Option<String> {
    let target = match s {
        "fn" => Tokens::FUNCTION,
        "let" => Tokens::LET,
        "true" => Tokens::TRUE,
        "false" => Tokens::FALSE,
        "if" => Tokens::IF,
        "else" => Tokens::ELSE,
        "return" => Tokens::RETURN,
        _ => " ",
    };

    if target == " " {
        return None;
    }

    Some(target.to_string())
}

pub fn lookup_ident(ident: &str) -> String {
    match keyword_to_token(ident) {
        Some(token) => token,
        None => Tokens::IDENT.to_string(),
    }
}
