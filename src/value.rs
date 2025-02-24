use std::{fmt, rc::Rc};

use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Function(Rc<str>, Vec<Rc<str>>, Box<Node>),
    NativeFunction(fn(&Vec<Value>) -> Value),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Function(name, _, _) => write!(f, "<fn {}>", name),
            Self::NativeFunction(_) => write!(f, "<native fn>"),
        }
    }
}
