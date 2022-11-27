mod error;
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{BlockStatement, Expression, Literal, Node, Statement},
    object::environment::{Env, Environment},
    object::Object,
    token::*,
};

use self::error::*;

thread_local!(static BOOLEAN_TRUE:Rc<Object> = Rc::new(Object::Boolean(true)));
thread_local!(static BOOLEAN_FALSE:Rc<Object> = Rc::new(Object::Boolean(false)));
thread_local!(static NULL:Rc<Object> = Rc::new(Object::Null));
pub fn eval(node: Node, env: Env) -> Result<Rc<Object>, EvalError> {
    match node {
        Node::Program(p) => eval_program(&p, env),
        Node::Stat(s) => eval_statements(&s, env),
        Node::Expr(e) => eval_expression(&e, env),
    }
}

fn eval_program(p: &Vec<Statement>, env: Env) -> Result<Rc<Object>, EvalError> {
    let mut res = access_null();
    for stmt in p {
        res = eval_statements(stmt, env.clone())?;

        if let Object::ReturnValue(_) = &*res {
            return Ok(res);
        }
    }
    Ok(res)
}

fn eval_statements(s: &Statement, env: Env) -> Result<Rc<Object>, EvalError> {
    match s {
        Statement::Expression(expr) => eval_expression(expr, env.clone()),
        Statement::Return(expr) => {
            let expr = eval_expression(&expr, env.clone())?;
            return Ok(Rc::new(Object::ReturnValue(expr)));
        }
        Statement::Let(identifier, expr) => {
            let value = eval_expression(&expr, env.clone())?;
            env.borrow_mut().set(identifier, value.clone());

            Ok(value)
        }
    }
}

fn eval_block_statements(statements: &BlockStatement, env: Env) -> Result<Rc<Object>, EvalError> {
    let mut res = access_null();

    for s in &statements.0 {
        res = eval_statements(&s, env.clone())?;

        if let Object::ReturnValue(_) = *res {
            return Ok(res);
        }
    }

    Ok(res)
}

fn eval_expression(e: &Expression, env: Env) -> Result<Rc<Object>, EvalError> {
    match e {
        Expression::Identifier(id) => eval_identifier(id, env.clone()),
        Expression::Literal(lit) => eval_literal(lit),
        Expression::Prefix(operator, expr) => {
            let right = eval_expression(expr, env.clone())?;
            return eval_prefix_expression(&operator, right.clone());
        }
        Expression::Infix(left, operator, right) => {
            let left = eval_expression(left, env.clone())?;
            let right = eval_expression(right, env.clone())?;
            return eval_infix_expression(left.clone(), operator, right.clone());
        }
        Expression::IfExpr(condition, consequence, alternative) => {
            return eval_if_expression(condition, consequence, alternative, env.clone());
        }
        Expression::Func(params, body) => {
            let params = params.clone();
            let body = body.clone();

            Ok(Rc::new(Object::Funtion(params, body, env.clone())))
        }
        Expression::FuncCall(expr, params) => {
            let func = eval_expression(expr, env.clone())?;
            let args = eval_expressions(params, env.clone())?;
            apply_function(func, &args)
        }
        _ => todo!(),
    }
}

fn eval_expressions(expressions: &Vec<Expression>, env: Env) -> Result<Vec<Rc<Object>>, EvalError> {
    let mut exprs = vec![];
    for expr in expressions {
        let res = eval_expression(expr, env.clone())?;
        exprs.push(res);
    }

    Ok(exprs)
}

fn eval_literal(lit: &Literal) -> Result<Rc<Object>, EvalError> {
    match lit {
        Literal::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Literal::Bool(b) => Ok(match_boolean_expression(b)),
        Literal::String(s) => Ok(Rc::new(Object::String(s.to_string()))),
    }
}

fn eval_identifier(id: &str, env: Env) -> Result<Rc<Object>, EvalError> {
     if let Some(obj) =env.borrow_mut().get(id){
        Ok(obj)
     }else if let Some(func) = Builtin::search(id){
        Ok(Rc::new(func))
     }else{
        Err(identifier_unfound::new(id))
     }
}

fn eval_prefix_expression(operator: &Token, right: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match operator {
        Token::BANG => eval_bang_operator_expression(right.clone()),
        Token::MINUS => eval_minus_prefix_operation(right.clone()),
        _ => Err(unknown_operator::prefix(operator, &right)),
    }
}

fn eval_bang_operator_expression(expr: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match *expr {
        Object::Boolean(b) => Ok(match_boolean_expression(&(!b))),
        Object::Integer(i) => {
            let b = if i == 0 { true } else { false };

            return Ok(match_boolean_expression(&b));
        }
        _ => Ok(match_boolean_expression(&false)),
    }
}

fn eval_minus_prefix_operation(expr: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match *expr {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(-i))),
        _ => Err(unknown_operator::minus_prefix(&expr)),
    }
}

fn eval_infix_expression(
    left_expr: Rc<Object>,
    operator: &Token,
    right_expr: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
    let left_val = &*left_expr;
    let right_val = &*right_expr;
    match (left_val, right_val) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(*left, operator, *right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(*left, operator, *right)
        }
        (Object::String(s1), Object::String(s2)) => {
            eval_string_infix_expression(s1, operator, s2)
        }
        _ => Err(type_mismatch::type_mismatch(
            &left_val.get_type(),
            operator,
            &right_val.get_type(),
        )),
    }
}

fn eval_integer_infix_expression(
    left: i64,
    operator: &Token,
    right: i64,
) -> Result<Rc<Object>, EvalError> {
    let res = match *operator {
        Token::PLUS => Object::Integer(left + right),
        Token::MINUS => Object::Integer(left - right),
        Token::ASTERISK => Object::Integer(left * right),
        Token::SLASH => Object::Integer(left / right),
        Token::GT => return Ok(match_boolean_expression(&(left > right))),
        Token::LT => return Ok(match_boolean_expression(&(left < right))),
        Token::EQ => return Ok(match_boolean_expression(&(left == right))),
        Token::NOTEQ => return Ok(match_boolean_expression(&(left != right))),
        _ => return Err(unknown_operator::infix(&left, operator, &right)),
    };

    Ok(Rc::new(res))
}

fn eval_boolean_infix_expression(
    left: bool,
    operator: &Token,
    right: bool,
) -> Result<Rc<Object>, EvalError> {
    match *operator {
        Token::EQ => return Ok(match_boolean_expression(&(left == right))),
        Token::NOTEQ => return Ok(match_boolean_expression(&(left != right))),

        _ => return Err(unknown_operator::infix(left, operator, right)),
    };
}

fn eval_string_infix_expression(
    s1: &str,
    operator: &Token,
    s2: &str,
) -> Result<Rc<Object>, EvalError> {
    match operator{
        Token::PLUS=>Ok(Rc::new(Object::String((s1.to_string()+s2)))),
        _=>Err(unknown_operator::infix(s1, operator, s2))
    }
}

fn eval_if_expression(
    condition: &Expression,
    consequence: &BlockStatement,
    alternative: &Option<BlockStatement>,
    env: Env,
) -> Result<Rc<Object>, EvalError> {
    let condition = eval_expression(condition, env.clone())?;
    if is_truthy(&condition) {
        return eval_block_statements(consequence, env.clone());
    } else {
        match &alternative {
            Some(alter) => eval_block_statements(&alter, env.clone()),
            None => Ok(access_null()),
        }
    }
}

fn apply_function(func: Rc<Object>, args: &Vec<Rc<Object>>) -> Result<Rc<Object>, EvalError> {
    match &*func {
        Object::Funtion(params, body, env) => {
            // let extended_env = Rc::new(RefCell::new(extend_func_env(&env, args, params)));
            let extended_env = match params {
                Some(params) => Rc::new(RefCell::new(extend_func_env(env.clone(), args, params))),
                None => env.clone(),
            };
            let evluated = eval_block_statements(&body, extended_env)?;

            unwrap_return_value(evluated)
        }
        _ => Err(type_mismatch::not_a_function(func)),
    }
}

fn extend_func_env(outer_env: Env, args: &Vec<Rc<Object>>, params: &Vec<String>) -> Environment {
    let mut env = Environment::new_enclosed_environment(outer_env.clone());
    for (i, param) in params.iter().enumerate() {
        env.set(param, args[i].clone())
    }

    env
}

fn unwrap_return_value(obj: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match &*obj {
        Object::ReturnValue(v) => Ok(v.clone()),
        _ => Ok(obj),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null | Object::Boolean(false) | Object::Integer(0) => false,
        _ => true,
    }
}

fn match_boolean_expression(b: &bool) -> Rc<Object> {
    match b {
        true => BOOLEAN_TRUE.with(|b| b.clone()),
        false => BOOLEAN_FALSE.with(|b| b.clone()),
    }
}

fn access_null() -> Rc<Object> {
    NULL.with(|n| n.clone())
}
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{object::environment, parser::start_parsing};

    use super::*;

    fn test_helper(cases: &[(&str, &str)]) {
        let env = Rc::new(RefCell::new(environment::Environment::new()));
        for (input, expected) in cases {
            let node = start_parsing(input).unwrap();
            match eval(node, env.clone()) {
                Ok(evaluated) => assert_eq!(expected, &format!("{}", evaluated)),
                Err(err) => assert_eq!(expected, &format!("{}", err)),
            }
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("-0", "0"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 + -50", "0"),
            ("5 * (2 + 10)", "60"),
            ("5 + 2 * 10", "25"),
            ("20 + 2 * -10", "0"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];

        test_helper(&tests)
    }

    #[test]
    fn test_bang_operator() {
        let tests = [
            ("!true", "false"),
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
        ];

        test_helper(&tests)
    }

    #[test]
    fn test_if_expressions() {
        let tests_non_null = [("if (10 > 1) {if (10 > 1) {return 10;}return 1;}", "10")];

        let tests_null = [("if(false){1}", "NULL"), ("if(1 > 2){true}", "NULL")];

        test_helper(&tests_non_null);
        test_helper(&tests_null);
    }

    #[test]
    fn test_error_handling() {
        let tests = [
            ("1+true", "Type Mismatch: Integer(1) + Boolean(true)"),
            ("false+2", "Type Mismatch: Boolean(false) + Integer(2)"),
            ("-true", "Unknown Operator: -true"),
            ("true + false", "Unknown Operator: true + false"),
            ("5; true + false; 5", "Unknown Operator: true + false"),
            (
                "if (10 > 1) { true + false; }",
                "Unknown Operator: true + false",
            ),
            (
                "if (10 > 1) {if (10 > 1) {return true + false;}return 1;}",
                "Unknown Operator: true + false",
            ),
            ("ggg", "Identifer not Found: ggg"),
        ];

        test_helper(&tests);
    }

    #[test]
    fn test_closure() {
        let tests = [(
            "let newAdder = fn(x) {fn(y) { x + y };};let addTwo = newAdder(2);addTwo(2);",
            "4",
        )];

        test_helper(&tests);
    }
}
