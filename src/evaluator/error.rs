// pub struct EvalError(String);

// impl fmt::Display for EvalError{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         return write!(f, "Error: {}", self.0);
//     }
// }

// impl EvalError{
//     pub fn new(err: String)->Self{
//         Self(err)
//     }

//     pub fn unknown_operator_error()
// }

use std::fmt;

use crate::{object::Object, token::Token};

pub enum EvalError {
    UnknownOperatorError
}

enum UnknownOperatorError<'a> {
    Prefix(&'a Token, &'a Object),
    MinusPrefix(&'a Object),
    IntegerBooleanPrefix(&'a i64, &'a Token, &'a i64),
}
impl fmt::Display for UnknownOperatorError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            UnknownOperatorError::Prefix(t, o) => format!("{}{}", t, o),
            UnknownOperatorError::MinusPrefix(o) => format!("-{}", o),
            UnknownOperatorError::IntegerBooleanPrefix(left, t, right) => {
                format!("{} {} {}", left, t, right)
            }
        };

        write!(f, "{}", res)
    }
}
