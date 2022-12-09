use crate::{object::Object, token::Token};
use std::{fmt, rc::Rc};
pub struct EvalError(String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}


pub fn prefix_error(t: &Token, o: &Object) -> EvalError {
    EvalError(format!("Unknown Operator:  {}{}", t, o))
}

pub fn minus_prefix_error(o: &Object) -> EvalError {
    EvalError(format!("Unknown Operator: -{}",  o))
}

pub fn infix_error<T: fmt::Display>(left: T, t: &Token, right: T) -> EvalError {
    EvalError(format!("Unknown Operator: {} {} {}", left, t, right))
}

pub fn type_mismatch(left: &str, t: &Token, right: &str) -> EvalError {
    EvalError(format!("Type Mismatch:  {} {} {}", left, t, right))
}

pub fn not_a_function(func: Rc<Object>) -> EvalError {
    EvalError(format!("{} is not a function", func))
}

pub fn argument_type_unsupported(obj: Rc<Object>, func_name: &str) -> EvalError {
    EvalError(format!(
        "Argument {} of type {} is not supported by function {}",
        obj,
        obj.get_type(),
        func_name
    ))
}

pub fn wrong_argument_number(func_name: &str, expect: i64, actual: i64) -> EvalError {
    EvalError(format!(
        "Function {} expected {} arguments, but got {}",
        func_name, expect, actual
    ))
}

pub fn operation_unsupported(obj: &Object) -> EvalError {
    EvalError(format!(
        "Object of type {} is not supported by this operation",
        obj.get_type()
    ))
}

pub fn identifier_unfound(id: &str) -> EvalError {
    EvalError(format!("Identifer not Found:  {}", id))
}

pub fn array_index_invalid(index: i64) -> EvalError {
    EvalError(format!("index {} is invalid", index))
}

pub fn array_index_out_of_bound(index: i64) -> EvalError {
    EvalError(format!("index {} exceeds the container length", index))
}

pub fn unhashable_expression(obj: &Object) -> EvalError {
    EvalError(format!("{} cannot be used as a hashkey", obj))
}
