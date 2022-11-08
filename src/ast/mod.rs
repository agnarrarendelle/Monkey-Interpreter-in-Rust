use std::fmt::{self, Display, format};

use crate::token::Token;

#[derive(Debug)]
pub enum Node {
    Stat(Statement),
    Expr(Expression),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Default for Program {
    fn default() -> Self {
        Self { statements: vec![] }
    }
}

impl fmt::Display for Program{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for stat in &self.statements{
            let stat_str = format!("{}",stat);
            res.push_str(&stat_str);
        }

        return  write!(f, "{}", res);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(identifier, expression) => {
                return write!(f, "let {} = {};", identifier, expression);
            }
            Statement::Return(expression) => {
                return write!(f, "return {};", expression);
            }
            Statement::Expression(expression) => {
                return write!(f, "{}", expression);
            }
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::Literal(x) => write!(f, "{}", x),
            Expression::Prefix(tok, expr) => write!(f, "({}{})", tok, expr),
            Expression::Infix(left_expr, tok, right_expr) => {
                write!(f, "({}{}{})", left_expr, tok, right_expr)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::write;

    use super::*;

    #[test]
    fn test_string() {
        let letStatement = Statement::Let(
            "myVar".to_string(),
            Expression::Identifier("anotherVar".to_string()),
        );

        let result = format!("{}", letStatement);
        assert_eq!(result, "let myVar = anotherVar;");
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal{
    Integer(i32),
    Bool(bool),
}

impl fmt::Display for Literal{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(int)=>write!(f, "{}",int),
            Self::Bool(bool)=>write!(f, "{}",bool)
        }
    }
}
