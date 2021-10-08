use std::collections::HashMap;

use crate::{
    built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn},
    types::MalType,
};

#[derive(Debug, Clone)]
pub struct MalEnv<'a> {
    data: HashMap<&'a str, MalType<'a>>,
    outer: Option<Box<MalEnv<'a>>>,
}

impl<'a> MalEnv<'a> {
    pub fn new() -> Self {
        MalEnv {
            data: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&MalType<'a>> {
        self.data.get(key)
    }

    pub fn repl() -> Self {
        let mut data = HashMap::new();
        let plus = plus_fn();
        data.insert(plus.name, MalType::Function(plus));
        let minus = minus_fn();
        data.insert(minus.name, MalType::Function(minus));
        let multiply = multiply_fn();
        data.insert(multiply.name, MalType::Function(multiply));
        let divide = divide_fn();
        data.insert(divide.name, MalType::Function(divide));

        MalEnv { data, outer: None }
    }
}

impl Default for MalEnv<'_> {
    fn default() -> Self {
        MalEnv::new()
    }
}
