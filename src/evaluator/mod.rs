use std::rc::Rc;

use crate::{
    ast::{Expression, Literal, Node, Statement},
    object::Object,
};

thread_local!(static BOOLEAN_TRUE:Rc<Object> = Rc::new(Object::Boolean(true)));
thread_local!(static BOOLEAN_FALSE:Rc<Object> = Rc::new(Object::Boolean(false)));
thread_local!(static NULL:Rc<Object> = Rc::new(Object::Null));
pub fn eval(node: Node) -> Rc<Object> {
    match node {
        Node::Program(p) => eval_program(p),
        Node::Stat(s) => eval_statements(s),
        Node::Expr(e) => eval_expression(e),
    }
}

fn eval_program(p: Vec<Statement>) -> Rc<Object> {
    let mut res = NULL.with( |n| n.clone());
    for stmt in p {
        res = eval_statements(stmt);
    }
    res
}

fn eval_statements(s: Statement) -> Rc<Object> {
    match s {
        Statement::Expression(expr) => eval_expression(expr),
        _ => todo!(),
    }
}

fn eval_expression(e: Expression) -> Rc<Object> {
    match e {
        Expression::Literal(lit) => eval_literal(lit),
        _ => todo!(),
    }
}

fn eval_literal(lit: Literal) -> Rc<Object> {
    match lit {
        Literal::Integer(i) => Rc::new(Object::Integer(i)),
        Literal::Bool(b) => match b {
            true => BOOLEAN_TRUE.with(|b| b.clone()),
            false => BOOLEAN_FALSE.with(|b| b.clone()),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast,
        lexer::Lexer,
        parser::{self, Parser},
    };

    use super::*;

    // #[test]
    // fn test_eval_integer_expression() {
    //     let tests = [("5", 5), ("10", 10)];

    //     for t in tests {
    //         let evaluated = testEval(t.0);
    //         assert_eq!(evaluated, Object::Integer(t.1))
    //     }
    // }

    // fn testEval(input: &str) -> Object {
    //     let l = Lexer::new(input);
    //     let mut p = Parser::new(l);
    //     let program = p.parse_program().unwrap();
    //     let node = Node::Program(program);

    //     eval(node)
    // }
}
