use crate::{BinaryOp, Node, UnaryOp, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(&mut self, ast: Node) -> Value {
        self.visit(ast)
    }

    fn visit(&mut self, node: Node) -> Value {
        match node {
            Node::Number(x) => Value::Number(x.parse().unwrap()),
            Node::Unary(op, node) => {
                let value = self.visit(*node);

                match op {
                    UnaryOp::Pos => value,
                    UnaryOp::Neg => match value {
                        Value::Number(x) => Value::Number(-x),
                    },
                }
            }
            Node::Binary(left, op, right) => {
                let l_value = self.visit(*left);
                let r_value = self.visit(*right);

                match op {
                    BinaryOp::Add => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                    },
                    BinaryOp::Sub => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                    },
                }
            }
            Node::Statements(nodes) => {
                let mut rtn_value = Value::Number(0.0);
                for node in nodes {
                    rtn_value = self.visit(node);
                }
                rtn_value
            }
            Node::EOF => Value::Number(0.0),
        }
    }
}
