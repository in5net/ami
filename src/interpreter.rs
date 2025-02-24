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
                    Value::NativeFunction(function) => Ok(function(&arg_values)),
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
            if args.len() != 1 {
                panic!("abs expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.abs()),
                _ => panic!("abs expects a number"),
            }
        });
        add_fn!("floor", |args| {
            if args.len() != 1 {
                panic!("floor expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.floor()),
                _ => panic!("floor expects a number"),
            }
        });
        add_fn!("ceil", |args| {
            if args.len() != 1 {
                panic!("ceil expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.ceil()),
                _ => panic!("ceil expects a number"),
            }
        });
        add_fn!("round", |args| {
            if args.len() != 1 {
                panic!("round expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.round()),
                _ => panic!("round expects a number"),
            }
        });
        add_fn!("trunc", |args| {
            if args.len() != 1 {
                panic!("trunc expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.trunc()),
                _ => panic!("trunc expects a number"),
            }
        });
        add_fn!("fract", |args| {
            if args.len() != 1 {
                panic!("fract expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.fract()),
                _ => panic!("fract expects a number"),
            }
        });
        add_fn!("sqrt", |args| {
            if args.len() != 1 {
                panic!("sqrt expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.cbrt()),
                _ => panic!("sqrt expects a number"),
            }
        });
        add_fn!("ln", |args| {
            if args.len() != 1 {
                panic!("ln expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.ln()),
                _ => panic!("ln expects a number"),
            }
        });
        add_fn!("sin", |args| {
            if args.len() != 1 {
                panic!("sin expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.sin()),
                _ => panic!("sin expects a number"),
            }
        });
        add_fn!("cos", |args| {
            if args.len() != 1 {
                panic!("cos expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.cos()),
                _ => panic!("cos expects a number"),
            }
        });
        add_fn!("tan", |args| {
            if args.len() != 1 {
                panic!("tan expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Number(value) => Value::Number(value.tan()),
                _ => panic!("tan expects a number"),
            }
        });
        add_fn!("gcd", |args| {
            if args.len() != 2 {
                panic!("gcd expects 2 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Number(a) => match args[1] {
                    Value::Number(b) => Value::Number(gcd(a, b)),
                    _ => panic!("gcd expects 2 integers"),
                },
                _ => panic!("gcd expects 2 integers"),
            }
        });
        add_fn!("lcm", |args| {
            if args.len() != 2 {
                panic!("lcm expects 2 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Number(a) => match args[1] {
                    Value::Number(b) => Value::Number(a * b / gcd(a, b)),
                    _ => panic!("lcm expects 2 integers"),
                },
                _ => panic!("lcm expects 2 integers"),
            }
        });
        add_fn!("min", |args| {
            if args.len() != 2 {
                panic!("min expects 2 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Number(a) => match args[1] {
                    Value::Number(b) => Value::Number(a.min(b)),
                    _ => panic!("min expects 2 numbers"),
                },
                _ => panic!("min expects 2 numbers"),
            }
        });
        add_fn!("max", |args| {
            if args.len() != 2 {
                panic!("max expects 2 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Number(a) => match args[1] {
                    Value::Number(b) => Value::Number(a.max(b)),
                    _ => panic!("max expects 2 numbers"),
                },
                _ => panic!("max expects 2 numbers"),
            }
        });
        add_fn!("clamp", |args| {
            if args.len() != 3 {
                panic!("clamp expects 3 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Number(a) => match args[1] {
                    Value::Number(b) => match args[2] {
                        Value::Number(c) => Value::Number(a.max(b).min(c)),
                        _ => panic!("clamp expects 3 numbers"),
                    },
                    _ => panic!("clamp expects 3 numbers"),
                },
                _ => panic!("clamp expects 3 numbers"),
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
