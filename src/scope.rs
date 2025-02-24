use std::{collections::HashMap, rc::Rc};

use crate::Value;

#[derive(Default)]
pub struct Scope {
    pub variables: HashMap<Rc<str>, Value>,
}

impl Scope {
    pub fn get(&self, name: &str) -> Value {
        match self.variables.get(name) {
            Some(value) => value.clone(),
            None => panic!("{} is not defined", name),
        }
    }

    pub fn set(&mut self, name: Rc<str>, value: Value) {
        self.variables.insert(name, value);
    }
}
