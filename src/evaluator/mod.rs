use crate::{
    ast::{Expression, Literal, Node, Statement},
    object::Object,
};


pub fn eval(node: Node) -> Object {
    match node {
        Node::Program(p) => eval_program(p),
        Node::Stat(s) => eval_statements(s),
        Node::Expr(e) => eval_expression(e),
    }
}

fn eval_program(p: Vec<Statement>) -> Object {
    let mut res = Object::Null;
    for stmt in p {
        res = eval_statements(stmt);
    }
    res
}

fn eval_statements(s: Statement) -> Object {
    match s {
        Statement::Expression(expr) => eval_expression(expr),
        _ => todo!(),
    }
}

fn eval_expression(e: Expression) -> Object {
    match e {
        Expression::Literal(Literal::Integer(i)) => Object::Integer(i),
        _ => todo!(),
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

    #[test]
    fn test_eval_integer_expression() {
        let tests = [("5", 5), ("10", 10)];

        for t in tests {
            let evaluated = testEval(t.0);
            assert_eq!(evaluated, Object::Integer(t.1))
        }
    }

    fn testEval(input: &str) -> Object {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let node = Node::Program(program);

        eval(node)
    }
}
