use std::ops::Range;

use crate::{AmiError, BinaryOp, Node, NodeType, Scope, UnaryOp, Value};

pub struct Interpreter {
    pub scope: Scope,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut interpreter = Self {
            scope: Scope::default(),
        };
        interpreter.add_builtins();
        interpreter
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
                    Value::NativeFunction(function) => match function(&arg_values) {
                        Ok(value) => Ok(value),
                        Err(reason) => self.error("".to_string(), reason, node.range),
                    },
                    _ => self.error(
                        format!("{} is not a function", name),
                        "".to_string(),
                        node.range,
                    ),
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

    fn add_builtins(&mut self) {
        macro_rules! add_var {
            ($name:literal, $value:expr) => {
                self.scope.set($name.into(), Value::from($value));
            };
        }

        add_var!("π", std::f64::consts::PI);
        add_var!("τ", std::f64::consts::TAU);
        add_var!("e", std::f64::consts::E);
        add_var!("𝜑", (1.0 + 5.0_f64.sqrt()) / 2.0);
        add_var!("𝜙", (1.0 + 5.0_f64.sqrt()) / 2.0);
        add_var!("∞", f64::INFINITY);

        macro_rules! add_fn {
            ($name:literal, $value:expr) => {
                self.scope.set($name.into(), Value::NativeFunction($value));
            };
        }

        add_fn!("abs", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.abs())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("floor", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.floor())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("ceil", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.ceil())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("round", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.round())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("trunc", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.trunc())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("fract", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.fract())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("sqrt", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.sqrt())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("cbrt", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.cbrt())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("ln", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.ln())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("sin", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.sin())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("cos", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.cos())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("tan", |args| {
            match args.get(0) {
                Some(Value::Number(value)) => Ok(Value::Number(value.tan())),
                _ => Err("expected a number".to_string()),
            }
        });
        add_fn!("gcd", |args| {
            match (args.get(0), args.get(1)) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Ok(Value::Number(gcd(*a, *b))),
                _ => Err("expected 2 numbers".to_string()),
            }
        });
        add_fn!("lcm", |args| {
            match (args.get(0), args.get(1)) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Ok(Value::Number(a * b / gcd(*a, *b)))
                }
                _ => Err("expected 2 numbers".to_string()),
            }
        });
        add_fn!("min", |args| {
            match (args.get(0), args.get(1)) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Ok(Value::Number(a.min(*b))),
                _ => Err("expected 2 numbers".to_string()),
            }
        });
        add_fn!("max", |args| {
            match (args.get(0), args.get(1)) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Ok(Value::Number(a.max(*b))),
                _ => Err("expected 2 numbers".to_string()),
            }
        });
        add_fn!("clamp", |args| {
            match (args.get(0), args.get(1), args.get(2)) {
                (Some(Value::Number(a)), Some(Value::Number(b)), Some(Value::Number(c))) => {
                    Ok(Value::Number(a.max(*b).min(*c)))
                }
                _ => Err("expected 2 numbers".to_string()),
            }
        });
    }
}

fn gcd(mut a: f64, mut b: f64) -> f64 {
    while b != 0.0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
