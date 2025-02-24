use std::ops::Range;

use crate::{AmiError, BinaryOp, Node, NodeType, Scope, UnaryOp, Value};

pub struct Interpreter {
    pub scope: Scope,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            scope: Scope::default(),
        }
    }
}

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
            NodeType::Identifier(name) => Ok(self.scope.get(&name)),
            NodeType::Assignment(name, node) => {
                let value = self.visit(*node)?;
                self.scope.set(name, value.clone());
                Ok(value)
            }
            NodeType::Unary(op, node) => {
                let value = self.visit(*node)?;

                match op {
                    UnaryOp::Pos => Ok(value),
                    UnaryOp::Neg => match value {
                        Value::Number(x) => Ok(Value::Number(-x)),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Sqrt => match value {
                        Value::Number(x) => Ok(Value::Number(x.sqrt())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Cbrt => match value {
                        Value::Number(x) => Ok(Value::Number(x.cbrt())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Fort => match value {
                        Value::Number(x) => Ok(Value::Number(x.powf(0.25))),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Degree => match value {
                        Value::Number(x) => Ok(Value::Number(x.to_radians())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Fact => match value {
                        Value::Number(x) => Ok(Value::Number({
                            let mut product = 0.0;
                            for n in 1..(x as i32) {
                                product *= n as f64;
                            }
                            product
                        })),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Abs => match value {
                        Value::Number(x) => Ok(Value::Number(x.abs())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Floor => match value {
                        Value::Number(x) => Ok(Value::Number(x.floor())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Ceil => match value {
                        Value::Number(x) => Ok(Value::Number(x.ceil())),
                        _ => unimplemented!(),
                    },
                    UnaryOp::Round => match value {
                        Value::Number(x) => Ok(Value::Number(x.round())),
                        _ => unimplemented!(),
                    },
                }
            }
            NodeType::Binary(left, op, right) => {
                let l_value = self.visit(*left)?;
                let r_value = self.visit(*right)?;

                match op {
                    BinaryOp::Add => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                        _ => unimplemented!(),
                    },
                    BinaryOp::Sub => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                        _ => unimplemented!(),
                    },
                    BinaryOp::Mul => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                        _ => unimplemented!(),
                    },
                    BinaryOp::Div => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                        _ => unimplemented!(),
                    },
                    BinaryOp::Mod => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
                        _ => unimplemented!(),
                    },
                    BinaryOp::Pow => match (l_value, r_value) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
                        _ => unimplemented!(),
                    },
                }
            }
            NodeType::FnDef(name, arg_names, node) => {
                let function = Value::Function(name.clone(), arg_names, node);
                self.scope.set(name, function.clone());
                Ok(function)
            }
            NodeType::Call(name, args) => {
                let mut arg_values: Vec<Value> = vec![];
                for arg in args {
                    let value = self.visit(arg)?;
                    arg_values.push(value);
                }

                let function = self.scope.get(&name);
                match function {
                    Value::Function(_, arg_names, body) => {
                        let mut interpreter = Interpreter::default();
                        for (name, value) in arg_names.iter().zip(arg_values) {
                            interpreter.scope.set(name.clone(), value.clone());
                        }
                        interpreter.visit(*body)
                    }
                    _ => panic!("{} is not a function", name),
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
