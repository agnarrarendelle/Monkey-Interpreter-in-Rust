use std::{fmt, rc::Rc};

use crate::object::Object;

use super::{error::*, check_container_index};

#[derive(PartialEq, Debug)]
pub enum Builtin {
    Len,
    First,
    Last
}

impl Builtin {
    pub fn search(func_name:&str)->Option<Object>{
        let res = match func_name{
            "len"=>Object::Builtin(Builtin::Len),
            "first"=>Object::Builtin(Builtin::First),
            "last"=>Object::Builtin(Builtin::Last),
            _=>return  None
        };

        Some(res)
    }

    pub fn apply(&self, args: &Vec<Rc<Object>>) -> Result<Rc<Object>, EvalError> {
        match self {
            Builtin::Len => {
                if args.len() != 1 {
                    return Err(type_mismatch::wrong_argument_number("len", 1, args.len() as i64));
                }

                match &*args[0] {
                    Object::String(s) => Ok(Rc::new(Object::Integer(s.len() as i64))),
                    Object::Array(arr) => Ok(Rc::new(Object::Integer(arr.len() as i64))),
                    _ => Err(type_mismatch::argument_type_unsupported(args[0].clone(), "len")),
                }
            },
            Builtin::First=>{
                if args.len() != 1 {
                    return Err(type_mismatch::wrong_argument_number("len", 1, args.len() as i64));
                }

                match &*args[0] {
                    Object::String(s) => {
                        check_container_index(0, s.len())?;
                        let char = s.chars().nth(0).unwrap();
                        Ok(Rc::new(Object::String(char.to_string())))
                    },
                    Object::Array(arr) => {
                        check_container_index(0, arr.len())?;
                        
                        Ok(arr[0].clone())
                    },
                    _ => Err(type_mismatch::argument_type_unsupported(args[0].clone(), "len")),
                }
            },
            Builtin::Last=>{
                if args.len() != 1 {
                    return Err(type_mismatch::wrong_argument_number("len", 1, args.len() as i64));
                }
                match &*args[0] {
                    Object::String(s) => {
                        check_container_index(0, s.len())?;
                        let char = s.chars().nth(s.len()-1).unwrap();
                        Ok(Rc::new(Object::String(char.to_string())))
                    },
                    Object::Array(arr) => {
                        check_container_index(0, arr.len())?;
                        
                        Ok(arr[arr.len()-1].clone())
                    },
                    _ => Err(type_mismatch::argument_type_unsupported(args[0].clone(), "len")),
                }
            }

        }
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
            Builtin::First => write!(f, "first"),
            Builtin::Last => write!(f, "last"),
        }
    }
}
