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
    use std::rc::Rc;

    use super::*;
    const ERROR_TYPE:&str = "Type Mismatch: ";
    pub fn type_mismatch(left:&str, t:&Token, right:&str)->EvalError{
        EvalError(format!("{}{} {} {}",ERROR_TYPE, left, t, right))
    }

    pub fn not_a_function(func: Rc<Object>)->EvalError{
        EvalError(format!("{} is not a function", func))
    }

    pub fn argument_type_unsupported(obj:Rc<Object>, func_name:&str)->EvalError{
        EvalError(format!("Argument {} of type {} is not supported by function {}", obj, obj.get_type(), func_name))
    }

    pub fn wrong_argument_number(func_name:&str, expect:i64, actual:i64)->EvalError{
        EvalError(format!("Function {} expected {} arguments, but got {}", func_name, expect, actual))
    }
}

pub mod identifier_unfound{
    use super::*;
    const ERROR_TYPE:&str = "Identifer not Found: ";
    pub fn new(id: &str)->EvalError{
        EvalError(format!("{}{}",ERROR_TYPE,id))
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
