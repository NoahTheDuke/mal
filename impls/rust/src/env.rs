use std::collections::HashMap;

use crate::{built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn}, symbol::Symbol, types::MalType};

#[derive(Debug, Clone)]
pub struct MalEnv<'a> {
    data: HashMap<String, MalType<'a>>,
    outer: Option<Box<MalEnv<'a>>>,
}

impl<'a> MalEnv<'a> {
    pub fn new() -> Self {
        MalEnv {
            data: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&MalType> {
        self.data.get(key)
    }

    pub fn get_sym(&self, key: &Symbol) -> Option<&MalType> {
        self.data.get(&key.name)
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

        MalEnv { data, outer: None }
    }
}

impl Default for MalEnv<'_> {
    fn default() -> Self {
        MalEnv::new()
    }
}
