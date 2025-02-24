use std::{fmt, rc::Rc};

use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Function(Rc<str>, Vec<Rc<str>>, Box<Node>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Function(name, _, _) => write!(f, "<fn {}>", name),
        }
    }
}
