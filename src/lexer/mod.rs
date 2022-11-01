use crate::token;
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
        use token::Tokens;
        let tok: token::Token;
        self.skip_whitespace();
        let ch = self.ch.to_string();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = new_token(Tokens::EQ.to_string(), "==")
                } else {
                    tok = new_token(Tokens::ASSIGN.to_string(), &ch)
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = new_token(Tokens::NOT_EQ.to_string(), "!=")
                } else {
                    tok = new_token(Tokens::BANG.to_string(), &ch)
                }
            }
            ';' => tok = new_token(Tokens::SEMICOLON.to_string(), &ch),
            '(' => tok = new_token(Tokens::LPAREN.to_string(), &ch),
            ')' => tok = new_token(Tokens::RPAREN.to_string(), &ch),
            ',' => tok = new_token(Tokens::COMMA.to_string(), &ch),
            '+' => tok = new_token(Tokens::PLUS.to_string(), &ch),
            '-' => tok = new_token(Tokens::MINUS.to_string(), &ch),
            '/' => tok = new_token(Tokens::SLASH.to_string(), &ch),
            '*' => tok = new_token(Tokens::ASTERISK.to_string(), &ch),
            '<' => tok = new_token(Tokens::LT.to_string(), &ch),
            '>' => tok = new_token(Tokens::GT.to_string(), &ch),
            '{' => tok = new_token(Tokens::LBRACE.to_string(), &ch),
            '}' => tok = new_token(Tokens::RBRACE.to_string(), &ch),
            ZERO_NULL => tok = new_token(Tokens::EOF.to_string(), "\0"),
            other => {
                if is_letter(other) {
                    let literal = self.read_identifier();
                    tok = new_token(token::lookup_ident(&literal), &literal);
                    return tok;
                } else if other.is_numeric() {
                    tok = new_token(Tokens::INT.to_string(), &self.read_number());
                    return tok;
                } else {
                    tok = new_token(Tokens::ILLEGAL.to_string(), &ch);
                }
            }
        }
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

fn new_token(token_type: token::TokenType, ch: &str) -> token::Token {
    token::Token::new(token_type, ch.to_string())
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        struct type_and_literal<'a> {
            expected_type: &'a str,
            expected_Literal: &'a str,
        }

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
            type_and_literal {
                expected_type: token::Tokens::LET,
                expected_Literal: "let",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "five",
            },
            type_and_literal {
                expected_type: token::Tokens::ASSIGN,
                expected_Literal: "=",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "5",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::LET,
                expected_Literal: "let",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "ten",
            },
            type_and_literal {
                expected_type: token::Tokens::ASSIGN,
                expected_Literal: "=",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::LET,
                expected_Literal: "let",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "add",
            },
            type_and_literal {
                expected_type: token::Tokens::ASSIGN,
                expected_Literal: "=",
            },
            type_and_literal {
                expected_type: token::Tokens::FUNCTION,
                expected_Literal: "fn",
            },
            type_and_literal {
                expected_type: token::Tokens::LPAREN,
                expected_Literal: "(",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "x",
            },
            type_and_literal {
                expected_type: token::Tokens::COMMA,
                expected_Literal: ",",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "y",
            },
            type_and_literal {
                expected_type: token::Tokens::RPAREN,
                expected_Literal: ")",
            },
            type_and_literal {
                expected_type: token::Tokens::LBRACE,
                expected_Literal: "{",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "x",
            },
            type_and_literal {
                expected_type: token::Tokens::PLUS,
                expected_Literal: "+",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "y",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::RBRACE,
                expected_Literal: "}",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::LET,
                expected_Literal: "let",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "result",
            },
            type_and_literal {
                expected_type: token::Tokens::ASSIGN,
                expected_Literal: "=",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "add",
            },
            type_and_literal {
                expected_type: token::Tokens::LPAREN,
                expected_Literal: "(",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "five",
            },
            type_and_literal {
                expected_type: token::Tokens::COMMA,
                expected_Literal: ",",
            },
            type_and_literal {
                expected_type: token::Tokens::IDENT,
                expected_Literal: "ten",
            },
            type_and_literal {
                expected_type: token::Tokens::RPAREN,
                expected_Literal: ")",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::BANG,
                expected_Literal: "!",
            },
            type_and_literal {
                expected_type: token::Tokens::MINUS,
                expected_Literal: "-",
            },
            type_and_literal {
                expected_type: token::Tokens::SLASH,
                expected_Literal: "/",
            },
            type_and_literal {
                expected_type: token::Tokens::ASTERISK,
                expected_Literal: "*",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "5",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "5",
            },
            type_and_literal {
                expected_type: token::Tokens::LT,
                expected_Literal: "<",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::GT,
                expected_Literal: ">",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "5",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::IF,
                expected_Literal: "if",
            },
            type_and_literal {
                expected_type: token::Tokens::LPAREN,
                expected_Literal: "(",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "5",
            },
            type_and_literal {
                expected_type: token::Tokens::LT,
                expected_Literal: "<",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::RPAREN,
                expected_Literal: ")",
            },
            type_and_literal {
                expected_type: token::Tokens::LBRACE,
                expected_Literal: "{",
            },
            type_and_literal {
                expected_type: token::Tokens::RETURN,
                expected_Literal: "return",
            },
            type_and_literal {
                expected_type: token::Tokens::TRUE,
                expected_Literal: "true",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::RBRACE,
                expected_Literal: "}",
            },
            type_and_literal {
                expected_type: token::Tokens::ELSE,
                expected_Literal: "else",
            },
            type_and_literal {
                expected_type: token::Tokens::LBRACE,
                expected_Literal: "{",
            },
            type_and_literal {
                expected_type: token::Tokens::RETURN,
                expected_Literal: "return",
            },
            type_and_literal {
                expected_type: token::Tokens::FALSE,
                expected_Literal: "false",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::RBRACE,
                expected_Literal: "}",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::EQ,
                expected_Literal: "==",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "10",
            },
            type_and_literal {
                expected_type: token::Tokens::NOT_EQ,
                expected_Literal: "!=",
            },
            type_and_literal {
                expected_type: token::Tokens::INT,
                expected_Literal: "9",
            },
            type_and_literal {
                expected_type: token::Tokens::SEMICOLON,
                expected_Literal: ";",
            },
            type_and_literal {
                expected_type: token::Tokens::EOF,
                expected_Literal: "\0",
            },
        ];

        let mut l = Lexer::new(input);

        for (index, elem) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                elem.expected_type, tok.token_type,
                "tests {} - tokentype wrong. expected {}, got {}",
                index, elem.expected_type, tok.token_type
            )
        }
    }
}
