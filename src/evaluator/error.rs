use std::fmt;
use crate::{token::Token, object::Object};
pub struct EvalError(String);

impl fmt::Display for EvalError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

pub mod unknown_operator{
    use super::*;
    const ERROR_TYPE:&str = "Unknown Operator: ";
    pub fn prefix(t: &Token, o: &Object)->EvalError{
        EvalError(format!("{}{}{}",ERROR_TYPE,t,o))
    }

    pub fn minus_prefix(o:&Object)->EvalError{
        EvalError(format!("{}-{}",ERROR_TYPE,o))
    }
    
    pub fn infix<T:fmt::Display>(left:T, t:&Token, right:T)->EvalError{
        EvalError(format!("{}{} {} {}",ERROR_TYPE, left, t, right))
    }
}

pub mod type_mismatch{
    use super::*;
    const ERROR_TYPE:&str = "Type Mismatch: ";
    pub fn type_mismatch(left:&str, t:&Token, right:&str)->EvalError{
        EvalError(format!("{}{} {} {}",ERROR_TYPE, left, t, right))
    }
}

// impl EvalError{
//     pub fn new(err: String)->Self{
//         Self(err)
//     }

//     pub fn unknown_operator_error()
// }

// use std::fmt;

// use crate::{object::Object, token::Token};

// pub enum EvalError<'a> {
//     UnknownOperator(UnknownOperatorError<'a>),
// }

// impl fmt::Display for EvalError<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self{
//             Self::UnknownOperator(e)=>write!(f, "{}", e)
            
//         }
//     }
// }

// pub enum UnknownOperatorError {
//     Prefix(& Token, & Object),
//     MinusPrefix(& Object),
//     IntegerBooleanPrefix(& Token, & Object),
//     IntegerInfix(i64, & Token, i64),
//     BooleanInfix(bool, & Token, bool)
// }
// impl fmt::Display for UnknownOperatorError<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let res = match self {
//             UnknownOperatorError::Prefix(t, o) => format!("{}{}", t, o),
//             UnknownOperatorError::MinusPrefix(o) => format!("-{}", o),
//             UnknownOperatorError::IntegerBooleanPrefix(t, o) => {
//                 format!("{} {}", t, o)
//             }
//             UnknownOperatorError::IntegerInfix(left, t, right) => {
//                 format!("{} {} {}", left, t, right)
//             },
//             UnknownOperatorError::BooleanInfix(left,t ,right )=>{
//                 format!("{} {} {}", left, t, right)
//             }
//         };

//         write!(f, "Unknown Operator Error: {}", res)
//     }
// }
