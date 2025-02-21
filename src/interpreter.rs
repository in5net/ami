use crate::{Node, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(&mut self, ast: Node) -> Value {
        self.visit(ast)
    }

    fn visit(&mut self, node: Node) -> Value {
        match node {
            Node::Number(x) => Value::Number(x.parse().unwrap()),
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
