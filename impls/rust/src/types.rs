use std::collections::HashMap;

use crate::{keyword::Keyword, symbol::Symbol};

#[derive(Clone, Debug)]
pub enum MalType<'a> {
    Atom(MalAtom),
    List(Vec<MalType<'a>>),
    Vector(Vec<MalType<'a>>),
    Map(HashMap<MalAtom, MalType<'a>>),
    Function(MalFunction<'a>),
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

#[derive(Clone, Copy)]
pub struct MalFunction<'a> {
    pub name: &'a str,
    f: fn(&[MalType]) -> Result<MalType<'a>, MalError>,
}

impl<'a> MalFunction<'a> {
    pub fn new(name: &'a str, f: fn(&[MalType]) -> Result<MalType<'a>, MalError>) -> Self {
        MalFunction { name, f }
    }

    pub fn invoke<'s>(&'s self, args: &[MalType]) -> Result<MalType<'a>, MalError> {
        (self.f)(args)
    }
}

#[derive(Clone, Debug)]
pub enum MalError {
    Normal(String),
    Parsing(String),
    Resolve(String),
}
