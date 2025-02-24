use std::ops::Range;

use crate::{AmiError, BinaryOp, Node, NodeType, UnaryOp, Value};

pub struct Interpreter {}

type RuntimeError = Result<Value, AmiError>;

impl Interpreter {
    fn error<T>(&self, msg: String, reason: String, range: Range<usize>) -> Result<T, AmiError> {
        Err(AmiError { msg, reason, range })
    }

    pub fn run(&mut self, ast: Node) -> RuntimeError {
        self.visit(ast)
    }

    fn visit(&mut self, node: Node) -> RuntimeError {
        match node.ty {
            NodeType::Number(x) => match x.parse::<f64>() {
                Ok(x) => Ok(Value::Number(x)),
                Err(e) => self.error(
                    format!("cannot parse '{}' as a number", x),
                    e.to_string(),
                    node.range,
                ),
            },
            NodeType::Unary(op, node) => {
                let value = self.visit(*node)?;

                match op {
                    UnaryOp::Pos => Ok(value),
                    UnaryOp::Neg => match value {
                        Value::Number(x) => Ok(Value::Number(-x)),
                    },
                }
            }
            NodeType::Binary(left, op, right) => {
                let l_value = self.visit(*left)?;
                let r_value = self.visit(*right)?;

                match op {
                    BinaryOp::Add => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    },
                    BinaryOp::Sub => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    },
                    BinaryOp::Mul => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    },
                    BinaryOp::Div => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                    },
                    BinaryOp::Mod => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
                    },
                    BinaryOp::Pow => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
                    },
                }
            }
            NodeType::Statements(nodes) => {
                let mut rtn_value = Value::Number(0.0);
                for node in nodes {
                    rtn_value = self.visit(node)?;
                }
                Ok(rtn_value)
            }
            NodeType::EOF => Ok(Value::Number(0.0)),
        }
    }
}
