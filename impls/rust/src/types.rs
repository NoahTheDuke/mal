use std::collections::HashMap;

use crate::built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn};

#[derive(Clone, Debug)]
pub enum MalType {
    Atom(MalAtom),
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    Map(HashMap<MalAtom, MalType>),
    Function(MalFunction),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MalAtom {
    Integer(isize),
    Symbol(String),
    Keyword(String),
    Str(String),
    Boolean(bool),
    Nil,
}

#[derive(Clone)]
pub struct MalFunction {
    pub name: String,
    f: fn(&[MalType]) -> Result<MalType, MalError>,
}

impl MalFunction {
    pub fn new(name: &str, f: fn(&[MalType]) -> Result<MalType, MalError>) -> Self {
        MalFunction {
            name: name.to_string(),
            f,
        }
    }

    pub fn invoke(&self, args: &[MalType]) -> Result<MalType, MalError> {
        (self.f)(args)
    }
}

#[derive(Clone, Debug)]
pub enum MalError {
    Normal(String),
    Parsing(String),
    Resolve(String),
}

#[derive(Debug, Clone)]
pub struct MalEnv {
    data: HashMap<String, MalType>,
}

impl MalEnv {
    pub fn new() -> Self {
        let mut data = HashMap::new();
        let plus = plus_fn();
        data.insert(plus.name.clone(), MalType::Function(plus));
        let minus = minus_fn();
        data.insert(minus.name.clone(), MalType::Function(minus));
        let multiply = multiply_fn();
        data.insert(multiply.name.clone(), MalType::Function(multiply));
        let divide = divide_fn();
        data.insert(divide.name.clone(), MalType::Function(divide));

        MalEnv { data }
    }

    pub fn get(&self, key: &str) -> Option<&MalType> {
        self.data.get(key)
    }
}

impl Default for MalEnv {
    fn default() -> Self {
        MalEnv::new()
    }
}
