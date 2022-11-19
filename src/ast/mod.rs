use crate::token::Token;
use std::fmt::{self};

pub enum Node {
    Program(Vec<Statement>),
    Stat(Statement),
    Expr(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Program(p) => write!(f, "{}", format(p, "")),
            Node::Stat(s) => write!(f, "{}", s),
            Node::Expr(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BlockStatement(pub Vec<Statement>);
impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for stat in &self.0 {
            res.push_str(&format!("{}", stat))
        }

        return write!(f, "{}", res);
    }
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
    IfExpr(Box<Expression>, BlockStatement, Option<BlockStatement>),
    Func(Option<Vec<String>>, BlockStatement),
    FuncCall(Box<Expression>, Vec<Expression>),
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
            Expression::IfExpr(condition, consequence, alternative) => {
                let condition_expr = format!("{}", condition);
                let consequence_expr = format!("{}", consequence);
                return match alternative {
                    Some(alter) => {
                        write!(
                            f,
                            "if {} {{ {} }} else {{ {} }}",
                            condition_expr, consequence_expr, alter
                        )
                    }
                    None => write!(f, "if {} {{ {} }}", condition_expr, consequence_expr),
                };
            }
            Expression::Func(params, body) => match params {
                Some(params) => {
                    write!(f, "fn({}) {{ {} }}", params.join(", "), body)
                }
                None => write!(f, "fn() {{ {} }}", body),
            },
            Expression::FuncCall(expression, arguments) => {
                write!(f, "{}({})", expression, format(&arguments, ", "))
            }
        };
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Integer(i64),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Bool(bool) => write!(f, "{}", bool),
        }
    }
}



fn format<T: fmt::Display>(f: &Vec<T>, seperator: &str) -> String {
    f.iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(seperator)
}
