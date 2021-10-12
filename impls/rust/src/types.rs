use std::collections::HashMap;

use crate::{keyword::Keyword, symbol::Symbol};

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
    Symbol(Symbol),
    Keyword(Keyword),
    Integer(isize),
    Str(String),
    Boolean(bool),
    Nil,
}

#[derive(Clone)]
pub struct MalFunction {
    pub name: Symbol,
    f: fn(Vec<MalType>) -> Result<MalType, MalError>,
}

impl MalFunction {
    pub fn new(name: Symbol, f: fn(Vec::<MalType>) -> Result<MalType, MalError>) -> Self {
        MalFunction { name, f }
    }

    pub fn invoke(&self, args: Vec::<MalType>) -> Result<MalType, MalError> {
        (self.f)(args)
    }
}

#[derive(Clone, Debug)]
pub enum MalError {
    Normal(String),
    Parsing(String),
    Resolve(String),
}
