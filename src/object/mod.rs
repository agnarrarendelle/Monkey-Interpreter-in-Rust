pub(crate) mod environment;
use std::{fmt, rc::Rc};

#[derive(PartialEq, Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Rc<Object>),
    Null
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::ReturnValue(v)=> write!(f, "{}", v),
            Object::Null => write!(f, "NULL"),
        }
    }
}

impl Object{
    pub fn get_type(&self)->String{
        match self {
            Object::Integer(i) => format!("Integer({})", i),
            Object::Boolean(b) => format!("Boolean({})", b),
            Object::ReturnValue(v)=> format!("{}", v),
            Object::Null => format!("NULL"),
        }
    }
}