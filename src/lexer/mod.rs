use crate::token::{self, Token};
const ZERO_NULL: char = 0 as char;
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input = input.chars().collect();
        let mut lex = Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: ZERO_NULL,
        };
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = ZERO_NULL;
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            ZERO_NULL
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let ch = self.ch;
        let tok = match ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                     Token::ASSIGN
                }
            },

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            },
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '/' => Token::SLASH,
            '*' => Token::ASTERISK,
            '<' => Token::LT,
            '>' => Token::GT,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            ZERO_NULL => Token::EOF,
            other => {
                if is_letter(other) {
                    let literal = self.read_identifier();
                    return token::lookup_ident(&literal);
                } else if other.is_numeric() {
                    return Token::INT(self.read_number().parse().unwrap());
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_char();
        tok
    }
    fn read_identifier(&mut self) -> String {
        let curr_pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let s: String = (self.input[curr_pos..self.position]).iter().collect();
        s
    }

    fn read_number(&mut self) -> String {
        let curr_pos = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }

        let s: String = (self.input[curr_pos..self.position]).iter().collect();
        s
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}


fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);

        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;

        ";
        let tests = vec![
        Token::LET,
        Token::IDENT("five".to_string()),
        Token::ASSIGN,
        Token::INT(5),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("ten".to_string()),
        Token::ASSIGN,
        Token::INT(10),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("add".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("x".to_string()),
        Token::COMMA,
        Token::IDENT("y".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENT("x".to_string()),
        Token::PLUS,
        Token::IDENT("y".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("result".to_string()),
        Token::ASSIGN,
        Token::IDENT("add".to_string()),
        Token::LPAREN,
        Token::IDENT("five".to_string()),
        Token::COMMA,
        Token::IDENT("ten".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::BANG,
        Token::MINUS,
        Token::SLASH,
        Token::ASTERISK,
        Token::INT(5),
        Token::SEMICOLON,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::GT,
        Token::INT(5),
        Token::SEMICOLON,
        Token::IF,
        Token::LPAREN,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::RPAREN,
        Token::LBRACE,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::ELSE,
        Token::LBRACE,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::INT(10),
        Token::EQ,
        Token::INT(10),
        Token::SEMICOLON,
        Token::INT(10),
        Token::NOTEQ,
        Token::INT(9),
        Token::SEMICOLON,
        Token::EOF,
        ];

        let mut l = Lexer::new(input);

        for (_, elem) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                *elem, tok
            )
        }
    }
}
