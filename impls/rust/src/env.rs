use std::collections::HashMap;

use crate::{
    built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn},
    symbol::Symbol,
    types::{MalError, MalType},
};

type SymbolTable = HashMap<String, MalType>;

#[derive(Debug, Clone)]
pub struct Env {
    data: SymbolTable,
    outer: Vec<SymbolTable>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            data: SymbolTable::new(),
            outer: Vec::new(),
        }
    }

    pub fn find(&self, key: &Symbol) -> Option<MalType> {
        if let Some(val) = self.data.get(&key.get_name()) {
            return Some(val.clone());
        } else if !self.outer.is_empty() {
            for symbol_table in self.outer.iter().rev() {
                if let Some(val) = symbol_table.get(&key.get_name()) {
                    return Some(val.clone());
                }
            }
        }
        None
    }

    pub fn get(&self, key: &Symbol) -> Result<MalType, MalError> {
        self.find(key)
            .ok_or_else(|| MalError::Resolve(key.get_name()))
    }

    pub fn set(&mut self, key: Symbol, val: MalType) -> MalType {
        self.data.insert(key.get_name(), val.clone());
        val
    }

    pub fn push_layer(&mut self) {
        self.outer.push(self.data.clone());
        self.data = SymbolTable::new();
    }

    pub fn pop_layer(&mut self) {
        if let Some(next_layer) = self.outer.pop() {
            self.data = next_layer;
        }
    }

    pub fn repl() -> Self {
        let mut env = Env::new();
        let plus = plus_fn();
        env.set(plus.name.clone(), MalType::Function(plus));
        let minus = minus_fn();
        env.set(minus.name.clone(), MalType::Function(minus));
        let multiply = multiply_fn();
        env.set(multiply.name.clone(), MalType::Function(multiply));
        let divide = divide_fn();
        env.set(divide.name.clone(), MalType::Function(divide));
        env
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}
