pub(crate) mod environment;
use std::{fmt, rc::Rc};

use crate::ast::BlockStatement;

use self::environment::Env;

#[derive(PartialEq, Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Rc<Object>),
    Funtion(Option<Vec<String>>, BlockStatement, Env),
    Null
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s)=>write!(f, "{}", s),
            Object::ReturnValue(v)=> write!(f, "{}", v),
            Object::Funtion(params, body, _)=>{
                match params{
                    Some(params)=> writeln!(f, "fn({}) {{\n{}\n}}", params.join(", "), body),
                    None=>writeln!(f, "fn() {{\n{}\n}}", body),
                }
            }
            Object::Null => write!(f, "NULL"),
        }
    }
}

impl Object{
    pub fn get_type(&self)->String{
        match self {
            Object::Integer(i) => format!("Integer({})", i),
            Object::Boolean(b) => format!("Boolean({})", b),
            Object::String(s) => format!("String(\"{}\")", s),
            Object::ReturnValue(v)=> format!("{}", v),
            Object::Funtion(params, body, _)=>{
                match params{
                    Some(params)=> format!("Function({}) {{\n{}\n}}", params.join(", "), body),
                    None=>format!("Function() {{\n{}\n}}", body),
                }
            }
            Object::Null => format!("NULL"),
        }
    }
}