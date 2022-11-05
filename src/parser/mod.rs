use crate::{ast::*, lexer::Lexer, token::*};

mod error;
use self::error::ParseError;
struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors = vec![];
        Parser {
            lexer,
            curr_token,
            peek_token,
            errors,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut program = Program::default();
        while !self.curr_token_is(&Token::EOF) {
            match self.parse_statement() {
                Ok(stat) => program.statements.push(stat),
                Err(err) => self.errors.push(err),
            }

            if(self.errors.len()!=0){
                return Err(self.errors.clone());
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.curr_token {
            Token::LET => self.parse_let_statements(),
            Token::RETURN => self.parse_return_statements(),
            _ => !unreachable!(),
        }
    }

    fn parse_let_statements(&mut self) -> Result<Statement, ParseError> {
        if let Token::IDENT(identifier) =  &self.peek_token {
            let identifier = identifier.clone();
            
            self.next_token();

            self.expect_peek_token(&Token::ASSIGN)?;
            self.next_token();

            while !self.curr_token_is(&Token::SEMICOLON) {
                self.next_token();
            }

            Ok(Statement::Let(
                identifier.clone(),
                Expression::Identifier("test".to_string()),
            ))
        } else {
            Err(ParseError::parse_identifier_error(&self.peek_token))
        }
    }

    fn parse_return_statements(&mut self)-> Result<Statement, ParseError>{

        self.next_token();
        while !self.curr_token_is(&Token::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::Return(Expression::Identifier("test".to_string())))
    }

    fn curr_token_is(&self, token_type: &Token) -> bool {
        self.curr_token == *token_type
    }

    fn peek_token_is(&self, token_type: &Token) -> bool {
        self.peek_token == *token_type
    }


    fn expect_peek_token(&mut self, token_type: &Token) -> Result<(), ParseError> {
        if (self.peek_token_is(token_type)) {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::parse_next_token_error(
                token_type,
                &self.peek_token,
            ))
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
        
        let x = 5;
        let y = 10;
        let foobar = 16666;

        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        test_parser_errors(&parser);
        let statements = program.unwrap().statements;
        assert_eq!(statements.len(), 3);
        let identifiers = vec!["x", "y", "foobar"];
        for (index, stat) in statements.iter().enumerate() {
            test_let_statement(stat, identifiers[index]);
        }
    }

    fn test_let_statement(stat: &Statement, identifier_name: &str) {
        matches!(stat, Statement::Let(_, _));
        match stat {
            Statement::Let(id, _) => {
                assert_eq!(id, identifier_name)
            },
            _=>!unreachable!()
        }
    }

    #[test]
    fn test_return_statement(){
        let input = "
        return 5;
        return 10;
        return 993322;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        test_parser_errors(&parser);
        let statements = program.unwrap().statements;
        assert_eq!(statements.len(), 3);

        for stat in &statements{
            assert!(matches!(stat, Statement::Return(_)), "expected ast::Statement::Return but got {}", stat);
        }
    }

    fn test_parser_errors(parser: &Parser) {
        if parser.errors.len() == 0 {
            return;
        }

        for e in &parser.errors{
            eprintln!("error: {}",e);
        }
        panic!();
    }
}
