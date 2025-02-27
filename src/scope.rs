use std::{collections::HashMap, rc::Rc};

use crate::Value;

#[derive(Default)]
pub struct Scope<'a> {
    pub variables: HashMap<Rc<str>, Value>,
    pub parent: Option<&'a Scope<'a>>,
}

impl Scope<'_> {
    pub fn get(&self, name: &str) -> Value {
        if let Some(value) = self.variables.get(name) {
            return value.clone();
        }

        if let Some(parent) = self.parent {
            return parent.get(name);
        }

        Value::Number(0.0)
    }

    pub fn set(&mut self, name: Rc<str>, value: Value) {
        self.variables.insert(name, value);
    }
}
