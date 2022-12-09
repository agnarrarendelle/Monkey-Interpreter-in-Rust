use std::{fmt, rc::Rc};

use crate::object::Object;

use super::{check_container_index, error::*, access_null};

#[derive(PartialEq, Debug, Clone)]
pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    Put
}

impl Builtin {
    pub fn search(func_name: &str) -> Option<Object> {
        let res = match func_name {
            "len" => Object::Builtin(Builtin::Len),
            "first" => Object::Builtin(Builtin::First),
            "last" => Object::Builtin(Builtin::Last),
            "rest" => Object::Builtin(Builtin::Rest),
            "push" => Object::Builtin(Builtin::Push),
            "put" => Object::Builtin(Builtin::Put),
            _ => return None,
        };

        Some(res)
    }

    pub fn apply(&self, args: &Vec<Rc<Object>>) -> Result<Rc<Object>, EvalError> {
        match self {
            Builtin::Len => {
                if args.len() != 1 {
                    return Err(wrong_argument_number(
                        "len",
                        1,
                        args.len() as i64,
                    ));
                }

                match &*args[0] {
                    Object::String(s) => Ok(Rc::new(Object::Integer(s.len() as i64))),
                    Object::Array(arr) => Ok(Rc::new(Object::Integer(arr.len() as i64))),
                    _ => Err(argument_type_unsupported(
                        args[0].clone(),
                        "len",
                    )),
                }
            }
            Builtin::First =>first(args),
            Builtin::Last => last(args),
            Builtin::Rest => rest(args),
            Builtin::Push => push(args),
            Builtin::Put=>put(args)
        }
    }
}

fn first(args: &Vec<Rc<Object>>)-> Result<Rc<Object>, EvalError>{
    if args.len() != 1 {
        return Err(wrong_argument_number(
            "first",
            1,
            args.len() as i64,
        ));
    }

    match &*args[0] {
        Object::String(s) => {
            check_container_index(0, s.len())?;
            let char = s.chars().nth(0).unwrap();
            Ok(Rc::new(Object::String(char.to_string())))
        }
        Object::Array(arr) => {
            check_container_index(0, arr.len())?;

            Ok(arr[0].clone())
        }
        _ => Err(argument_type_unsupported(
            args[0].clone(),
            "first",
        )),
    }
}

fn last(args: &Vec<Rc<Object>>)-> Result<Rc<Object>, EvalError>{
    if args.len() != 1 {
        return Err(wrong_argument_number(
            "last",
            1,
            args.len() as i64,
        ));
    }
    match &*args[0] {
        Object::String(s) => {
            check_container_index(0, s.len())?;
            let char = s.chars().nth(s.len() - 1).unwrap();
            Ok(Rc::new(Object::String(char.to_string())))
        }
        Object::Array(arr) => {
            check_container_index(0, arr.len())?;

            Ok(arr[arr.len() - 1].clone())
        }
        _ => Err(argument_type_unsupported(
            args[0].clone(),
            "last",
        )),
    }
}

fn rest(args: &Vec<Rc<Object>>)-> Result<Rc<Object>, EvalError>{
    if args.len() != 1 {
        return Err(wrong_argument_number(
            "rest",
            1,
            args.len() as i64,
        ));
    }

    match &*args[0] {
        Object::String(s) => Ok(Rc::new(Object::String(s[1..].to_string()))),
        Object::Array(arr) => {
            let mut rest_arr = vec![];
            for i in 1..arr.len() {
                let copied_obj = Rc::new((*(arr[i])).clone());
                rest_arr.push(copied_obj)
            }
            Ok(Rc::new(Object::Array(rest_arr)))
        }
        _ => Err(argument_type_unsupported(
            args[0].clone(),
            "rest",
        )),
    }
}

fn push(args: &Vec<Rc<Object>>)-> Result<Rc<Object>, EvalError>{
    if args.len() != 2 {
        return Err(wrong_argument_number(
            "push",
            2,
            args.len() as i64,
        ));
    }
    let new_elem = Rc::new((*args[1]).clone());
    match &*args[0] {
        Object::String(s) => match &*new_elem {
            Object::String(new_elem) => {
                let mut new_str = s.clone();
                new_str.push_str(&new_elem);
                return Ok(Rc::new(Object::String(new_str)))
            }
            _ => Err(operation_unsupported(&new_elem)),
        },
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(new_elem);
            Ok(Rc::new(Object::Array(new_arr)))
        }
        _ => Err(argument_type_unsupported(
            args[0].clone(),
            "push",
        )),
    }
}

fn put(args: &Vec<Rc<Object>>)-> Result<Rc<Object>, EvalError>{
   for arg in args{
    println!("{} ", arg)
   };
   
   Ok(access_null())
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
            Builtin::First => write!(f, "first"),
            Builtin::Last => write!(f, "last"),
            Builtin::Rest => write!(f, "rest"),
            Builtin::Push => write!(f, "push"),
            Builtin::Put => write!(f, "put"),
        }
    }
}
