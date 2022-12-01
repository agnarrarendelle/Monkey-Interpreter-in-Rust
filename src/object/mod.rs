pub(crate) mod environment;
use std::collections::BTreeMap;
use std::{fmt, rc::Rc};
use std::hash::{Hash, Hasher};

use self::environment::Env;
use crate::ast::BlockStatement;
use crate::evaluator::builtins::Builtin;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Rc<Object>),
    Funtion(Option<Vec<String>>, BlockStatement, Env),
    Builtin(Builtin),
    Array(Vec<Rc<Object>>),
    Hash(BTreeMap<Rc<Object>, Rc<Object>>),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::ReturnValue(v) => write!(f, "{}", v),
            Object::Funtion(params, body, _) => match params {
                Some(params) => writeln!(f, "fn({}) {{\n{}\n}}", params.join(", "), body),
                None => writeln!(f, "fn() {{\n{}\n}}", body),
            },
            Object::Array(elems) => writeln!(f, "[{}]", get_array_element_string(elems)),
            Object::Hash(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{{{}}}", map)
            }
            Object::Builtin(b) => write!(f, "Builtin Function: {}", b),
            Object::Null => write!(f, "NULL"),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Integer(i) => format!("Integer({})", i),
            Object::Boolean(b) => format!("Boolean({})", b),
            Object::String(s) => format!("String(\"{}\")", s),
            Object::ReturnValue(v) => format!("{}", v),
            Object::Funtion(params, body, _) => match params {
                Some(params) => format!("Function({}) {{\n{}\n}}", params.join(", "), body),
                None => format!("Function() {{\n{}\n}}", body),
            },
            Object::Array(elems) => format!("Array[{}]", get_array_element_string(elems)),
            Object::Hash(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("Map{{{}}}", map)
            }
            Object::Builtin(b) => format!("Builtin Function {}", b),
            Object::Null => format!("NULL"),
        }
    }

    pub fn is_hashtable(&self)->bool{
        match self{
            Self::Integer(_) | Self::Boolean(_) | Self::String(_) => true,
            _ => false,
        }
    }
}

impl Hash for Object{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self{
            Object::Integer(i)=>i.hash(state),
            Object::String(s)=>s.hash(state),
            Object::Boolean(b)=>b.hash(state),
            _=>!unreachable!()
        }
    }
}

fn get_array_element_string(elems: &Vec<Rc<Object>>) -> String {
    let mut arr = vec![];
    for e in elems {
        arr.push(format!("{}", e))
    }

    arr.join(", ")
}
