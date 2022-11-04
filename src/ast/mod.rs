use std::fmt::{Display, self};

#[derive(Debug)]
pub enum Node{
    Stat(Statement),
    Expr(Expression)
}

#[derive(Debug)]
pub struct Program{
    pub statements: Vec<Statement>
}


impl Default for Program {
    fn default() -> Self {
        Self{
            statements:vec![]
        }
    }
}


#[derive(Debug)]
pub enum Statement{
    Let(String, Expression),
}

impl fmt::Display for Statement{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Statement::Let(identifier,expression )=>{
                return  write!(f, "let {} = {};", identifier, expression);
            }
        }
    }
}
#[derive(Debug)]
pub enum Expression{
    Identifier(String)
}

impl fmt::Display for Expression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Expression::Identifier(identifier)=>{
                return  write!(f, "{}", identifier);
            }
        }
    }
}