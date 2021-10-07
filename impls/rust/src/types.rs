use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum MalAtom {
    Integer(isize),
    Symbol(String),
    Keyword(String),
    Str(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum MalType {
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    Map(HashMap<MalAtom, MalType>),
    Atom(MalAtom),
}

#[derive(Debug)]
pub enum MalError {
    Normal(String),
    Parsing(String),
}
