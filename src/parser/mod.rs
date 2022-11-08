use crate::{ast::*, lexer::Lexer, token::*};

mod error;
mod precedence;
use self::error::ParseError;
use precedence::*;
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

    pub fn start_parsing(input: &str)->Result<Program, Vec<ParseError>>{
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;

        Ok(program)
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

            if (self.errors.len() != 0) {
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
            _ => self.parse_expression_statements(),
        }
    }

    fn parse_let_statements(&mut self) -> Result<Statement, ParseError> {
        if let Token::IDENT(identifier) = &self.peek_token {
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

    fn parse_return_statements(&mut self) -> Result<Statement, ParseError> {
        self.next_token();
        while !self.curr_token_is(&Token::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::Return(Expression::Identifier(
            "test".to_string(),
        )))
    }

    fn parse_expression_statements(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression(Precedence::LOWEST)?;
        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut left_expr = match &self.curr_token {
            Token::IDENT(x) => Ok(Expression::Identifier(x.clone())),
            Token::INT(x) => match x.parse::<i32>() {
                Ok(num) => Ok(Expression::IntegerLiteral(num)),
                Err(e) => Err(ParseError::parse_integer_error(x)),
            },
            Token::BANG | Token::MINUS => self.parse_prefix_expression(),
            _ => todo!(),
        };

        while !self.peek_token_is(&Token::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::PLUS
                | Token::MINUS
                | Token::ASTERISK
                | Token::SLASH
                | Token::EQ
                | Token::NOTEQ
                | Token::LT
                | Token::GT => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(left_expr.unwrap());
                }
                _ => todo!(),
            }
        }

        left_expr
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let curr_tok = self.curr_token.clone();
        self.next_token();
        let expr = self.parse_expression(Precedence::PREFIX)?;

        Ok(Expression::Prefix(curr_tok, Box::new(expr)))
    }

    fn parse_infix_expression(&mut self, left_expr: Expression) -> Result<Expression, ParseError> {
        let operand = self.curr_token.clone();
        let precedence = token_to_precedence(&operand);
        self.next_token();
        let right_expr = self.parse_expression(precedence)?;

        Ok(Expression::Infix(
            Box::new(left_expr),
            operand,
            Box::new(right_expr),
        ))
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

    fn peek_precedence(&self) -> Precedence {
        token_to_precedence(&self.peek_token)
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
            }
            _ => !unreachable!(),
        }
    }

    #[test]
    fn test_return_statement() {
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

        for stat in &statements {
            assert!(
                matches!(stat, Statement::Return(_)),
                "expected ast::Statement::Return but got {}",
                stat
            );
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        test_parser_errors(&parser);
        let statements = program.unwrap().statements;
        assert_eq!(statements.len(), 1);
        assert!(
            matches!(statements[0], Statement::Expression(_)),
            "expected Expression Statemnet but got {}",
            statements[0]
        );
        match &statements[0] {
            Statement::Expression(Expr) => {
                if let Expression::Identifier(x) = Expr {
                    assert_eq!(x, "foobar")
                }
            }
            _ => !unreachable!(),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        test_parser_errors(&parser);
        let statements = program.unwrap().statements;
        assert_eq!(statements.len(), 1);
        assert!(
            matches!(statements[0], Statement::Expression(_)),
            "expected Expression Statemnet but got {}",
            statements[0]
        );
        match &statements[0] {
            Statement::Expression(Expr) => {
                if let Expression::IntegerLiteral(x) = Expr {
                    assert_eq!(*x, 5);
                }
            }
            _ => !unreachable!(),
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let expressions = [("!5;", "!", "5", "(!5)"), ("-15;", "-", "15", "(-15)")];

        for expr in expressions {
            let lexer = Lexer::new(expr.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            test_parser_errors(&parser);
            let statements = program.unwrap().statements;
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Statement::Expression(expression) => match expression {
                    Expression::Prefix(tok, rightExpr) => {
                        let actual_expr = format!("{}", expression);
                        let actual_tok = format!("{}", tok);
                        let actual_right_expr = format!("{}", rightExpr);
                        assert_eq!(actual_tok, expr.1);
                        assert_eq!(actual_right_expr, expr.2);
                        assert_eq!(actual_expr, expr.3);
                    }
                    _ => panic!("not prefix expression"),
                },
                _ => panic!("not expression"),
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let expressions = [
            ("5+5;", "5", "+", "5", "(5+5)"),
            ("5-5;", "5", "-", "5", "(5-5)"),
            ("5*5;", "5", "*", "5", "(5*5)"),
            ("5/5;", "5", "/", "5", "(5/5)"),
            ("5>5;", "5", ">", "5", "(5>5)"),
            ("5<5;", "5", "<", "5", "(5<5)"),
            ("5==5;", "5", "==", "5", "(5==5)"),
            ("5!=5;", "5", "!=", "5", "(5!=5)"),
        ];

        for expr in expressions {
            let lexer = Lexer::new(expr.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            test_parser_errors(&parser);
            let statements = program.unwrap().statements;
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Statement::Expression(expression) => match expression {
                    Expression::Infix(left_expr, tok, right_expr) => {
                        let actual_expr = format!("{}", expression);
                        let actual_left = format!("{}", left_expr);
                        let actual_tok = format!("{}", tok);
                        let actual_right = format!("{}", right_expr);
                        assert_eq!(actual_left, expr.1);
                        assert_eq!(actual_tok, expr.2);
                        assert_eq!(actual_right, expr.3);
                        assert_eq!(actual_expr, expr.4);
                    }
                    _ => panic!("not infix expression"),
                },
                _ => panic!("not expression"),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing(){
        let tests = [
            ("-a * b","((-a)*b)"),
            ("!-a","(!(-a))"),
            ("a + b + c","((a+b)+c)"),
            ("a + b - c","((a+b)-c)"),
            ("a * b * c","((a*b)*c)"),
            ("a * b / c","((a*b)/c)"),
            ("a + b / c","(a+(b/c))"),
            ("a + b * c + d / e - f","(((a+(b*c))+(d/e))-f)"),
            ("3 + 4; -5 * 5","(3+4)((-5)*5)"),
            ("5 > 4 == 3 < 4","((5>4)==(3<4))"),
            ("5 < 4 != 3 > 4","((5<4)!=(3>4))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5","((3+(4*5))==((3*1)+(4*5)))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5","((3+(4*5))==((3*1)+(4*5)))"),
        ];

        for test in tests{
            let lexer = Lexer::new(test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            test_parser_errors(&parser);
            let program = program.unwrap();
            assert_eq!(format!("{}", program), test.1);
            // match &statements[0]{
            //     Statement::Expression(expression)=>{
            //         let actual_expr = format!("{}", expression);
            //         assert_eq!(test.1, actual_expr)
            //     },
            //     _=>panic!("not expression")
            // }
        }
    }

    fn test_parser_errors(parser: &Parser) {
        if parser.errors.len() == 0 {
            return;
        }

        for e in &parser.errors {
            eprintln!("error: {}", e);
        }
        panic!();
    }
}
