use crate::{ast::*, lexer::Lexer, token::*};

mod error;
mod precedence;
use self::error::ParseError;
use precedence::*;
pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

pub fn start_parsing(input: &str) -> Result<Node, Vec<ParseError>> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    Ok(Node::Program(program))
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

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, Vec<ParseError>> {
        let mut program = vec![];
        while !self.curr_token_is(&Token::EOF) {
            match self.parse_statement() {
                Ok(stat) => program.push(stat),
                Err(err) => self.errors.push(err),
            }

            self.next_token();
        }

        if self.errors.len() != 0 {
            return Err(self.errors.clone());
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.curr_token {
            Token::LET => self.parse_let_statements(),
            Token::RETURN => self.parse_return_statements(),
            Token::ILLEGAL => Err(ParseError::illegal_token_error()),
            _ => self.parse_expression_statements(),
        }
    }

    fn parse_let_statements(&mut self) -> Result<Statement, ParseError> {
        if let Token::IDENT(identifier) = &self.peek_token {
            let identifier = identifier.clone();

            self.next_token();

            self.expect_peek_token(&Token::ASSIGN)?;
            self.next_token();

            let expression = self.parse_expression(Precedence::LOWEST)?;

            if !self.curr_token_is(&Token::SEMICOLON) {
                self.next_token();
            }

            Ok(Statement::Let(identifier.clone(), expression))
        } else {
            Err(ParseError::parse_identifier_error(&self.peek_token))
        }
    }

    fn parse_return_statements(&mut self) -> Result<Statement, ParseError> {
        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if !self.curr_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Return(expression))
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
            Token::INT(x) => match x.parse::<i64>() {
                Ok(num) => Ok(Expression::Literal(Literal::Integer(num))),
                Err(_) => Err(ParseError::parse_integer_error(x)),
            },
            Token::BOOLEAN(b) => Ok(Expression::Literal(Literal::Bool(*b))),
            Token::BANG | Token::MINUS => self.parse_prefix_expression(),
            Token::LPAREN => self.parse_group_expression(),
            Token::IF => self.parse_if_expression(),
            Token::FUNCTION => self.parse_function_expression(),
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
                Token::LPAREN => {
                    self.next_token();
                    left_expr = self.parse_func_call_expression(left_expr.unwrap())
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

    fn parse_group_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek_token(&Token::RPAREN)?;
        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_peek_token(&Token::LPAREN)?;
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek_token(&Token::RPAREN)?;
        self.expect_peek_token(&Token::LBRACE)?;
        let consequence = self.parse_block_statements()?;
        let mut alternative = Option::None;
        if self.peek_token_is(&Token::ELSE) {
            self.next_token();
            self.expect_peek_token(&Token::LBRACE)?;
            alternative = Some(self.parse_block_statements()?);
        }

        Ok(Expression::IfExpr(
            Box::new(condition),
            consequence,
            alternative,
        ))
    }

    fn parse_block_statements(&mut self) -> Result<BlockStatement, ParseError> {
        let mut statements = vec![];
        self.next_token();
        while !self.curr_token_is(&Token::RBRACE) && !self.curr_token_is(&Token::EOF) {
            let stat = self.parse_statement()?;
            statements.push(stat);
            self.next_token();
        }
        Ok(BlockStatement(statements))
    }

    fn parse_function_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_peek_token(&Token::LPAREN)?;

        let params = self.parse_function_parameter()?;
        self.expect_peek_token(&Token::LBRACE);

        let body = self.parse_block_statements()?;

        let expr = match params {
            Some(params) => Expression::Func(Some(params), body),
            None => Expression::Func(None, body),
        };

        Ok(expr)
    }

    fn parse_function_parameter(&mut self) -> Result<Option<Vec<String>>, ParseError> {
        if self.peek_token_is(&Token::RPAREN) {
            self.next_token();
            return Ok(None);
        }
        let mut identifiers = vec![];
        self.next_token();

        match &self.curr_token {
            Token::IDENT(id) => identifiers.push(id.clone()),
            other => return Err(ParseError::parse_identifier_error(other)),
        }

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();

            match &self.curr_token {
                Token::IDENT(id) => identifiers.push(id.clone()),
                other => return Err(ParseError::parse_identifier_error(other)),
            }
        }

        self.expect_peek_token(&Token::RPAREN)?;

        Ok(Some(identifiers))
    }

    fn parse_func_call_expression(
        &mut self,
        expression: Expression,
    ) -> Result<Expression, ParseError> {
        let args = self.parse_func_call_arguments()?;
        Ok(Expression::FuncCall(Box::new(expression), args))
    }

    fn parse_func_call_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args: Vec<Expression> = vec![];

        self.next_token();
        args.push(self.parse_expression(Precedence::LOWEST)?);

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::LOWEST)?);
        }

        self.expect_peek_token(&Token::RPAREN)?;
        Ok(args)
    }

    fn curr_token_is(&self, token_type: &Token) -> bool {
        self.curr_token == *token_type
    }

    fn peek_token_is(&self, token_type: &Token) -> bool {
        self.peek_token == *token_type
    }

    fn expect_peek_token(&mut self, token_type: &Token) -> Result<(), ParseError> {
        if self.peek_token_is(token_type) {
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

    fn test_helper(cases: &Vec<(&str, &str)>) {
        for (input, expected) in cases {
            match start_parsing(input) {
                Ok(program) => {
                    assert_eq!(*expected, &format!("{}", program))
                }
                Err(errors) => {
                    println!("Errors: ");
                    for e in errors {
                        println!("{}", e);
                    }
                    panic!()
                }
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let test_cases = vec![
            ("let x = 5;", "let x = 5;"),
            ("let y = 10;", "let y = 10;"),
            ("let foobar = 16666;", "let foobar = 16666;"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_return_statement() {
        let test_cases = vec![
            ("return 5;", "return 5;"),
            ("return -5;", "return (-5);"),
            ("return foo;", "return foo;"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_identifier_expression() {
        let test_cases = vec![
            ("foo;", "foo"),
            ("foo_bar;", "foo_bar"),
            ("123_test;", "123_test"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_integer_literal_expression() {
        let test_cases = vec![("5;", "5"), ("-15;", "(-15)")];

        test_helper(&test_cases);
    }

    #[test]
    fn test_boolean_literal_expression() {
        let test_cases = vec![
            ("true;", "true"),
            ("false;", "false"),
            ("let foo = true;", "let foo = true;"),
            ("3 > 5 == false", "((3>5)==false)"),
            ("!true", "(!true)"),
            ("!!false", "(!(!false))"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let test_cases = vec![("!5;", "(!5)"), ("-15;", "(-15)")];

        test_helper(&test_cases);
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let test_cases = vec![
            ("5+5;", "(5+5)"),
            ("5-5;", "(5-5)"),
            ("5*5;", "(5*5)"),
            ("5/5;", "(5/5)"),
            ("5>5;", "(5>5)"),
            ("5<5;", "(5<5)"),
            ("5==5;", "(5==5)"),
            ("5!=5;", "(5!=5)"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let test_cases = vec![
            ("-a * b", "((-a)*b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a+b)+c)"),
            ("a + b - c", "((a+b)-c)"),
            ("a * b * c", "((a*b)*c)"),
            ("a * b / c", "((a*b)/c)"),
            ("a + b / c", "(a+(b/c))"),
            ("a + b * c + d / e - f", "(((a+(b*c))+(d/e))-f)"),
            ("3 + 4; -5 * 5", "(3+4)((-5)*5)"),
            ("5 > 4 == 3 < 4", "((5>4)==(3<4))"),
            ("5 < 4 != 3 > 4", "((5<4)!=(3>4))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3+(4*5))==((3*1)+(4*5)))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3+(4*5))==((3*1)+(4*5)))"),
            ("(3+4)*2", "((3+4)*2)"),
            ("4*(4/2)", "(4*(4/2))"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_if_else_block() {
        let test_cases = vec![
            ("if(x<y){x};", "if (x<y) { x }"),
            ("if ( x < y ) { x } else { y }", "if (x<y) { x } else { y }"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_func_paramater_parsing() {
        let test_cases = vec![
            ("fn() {};", "fn() {  }"),
            ("fn(x) {};", "fn(x) {  }"),
            ("fn(x,y,z) {};", "fn(x, y, z) {  }"),
        ];

        test_helper(&test_cases);
    }

    #[test]
    fn test_func_call_expression() {
        let test_cases = vec![
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2*3), (4+5), add(6, (7*8)))",
            ),
            ("add(1, 2 * 3, 4 + 5);", "add(1, (2*3), (4+5))"),
        ];
        test_helper(&test_cases);
    }
}
