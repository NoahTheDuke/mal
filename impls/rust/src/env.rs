use std::collections::HashMap;

use crate::{
    built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn},
    symbol::Symbol,
    types::{MalError, MalType},
};

#[derive(Debug, Clone)]
pub struct Env {
    data: HashMap<String, MalType>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            data: HashMap::new(),
            outer: None,
        }
    }

    pub fn find(&self, key: &Symbol) -> Option<MalType> {
        if let Some(val) = self.data.get(&key.get_name()) {
            Some(val.clone())
        } else if let Some(outer) = &self.outer {
            (*outer).find(key)
        } else {
            None
        }
    }

    pub fn get(&self, key: &Symbol) -> Result<MalType, MalError> {
        self.find(key)
            .ok_or_else(|| MalError::Resolve(key.get_name()))
    }

    pub fn set(&mut self, key: Symbol, val: MalType) -> Option<MalType> {
        self.data.insert(key.get_name(), val)
    }

    pub fn repl() -> Self {
        let mut data = HashMap::new();
        let plus = plus_fn();
        data.insert(plus.name.to_string(), MalType::Function(plus));
        let minus = minus_fn();
        data.insert(minus.name.to_string(), MalType::Function(minus));
        let multiply = multiply_fn();
        data.insert(multiply.name.to_string(), MalType::Function(multiply));
        let divide = divide_fn();
        data.insert(divide.name.to_string(), MalType::Function(divide));

        Env { data, outer: None }
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}
