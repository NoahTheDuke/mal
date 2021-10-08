use crate::types::{MalAtom, MalError, MalFunction, MalType};
use std::{fmt, result};

impl fmt::Display for MalAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalAtom::Symbol(s) => write!(f, "{}", s),
            MalAtom::Keyword(k) => write!(f, "{}", k),
            MalAtom::Integer(i) => write!(f, "{}", i),
            MalAtom::Str(s) => write!(f, "{}", s),
            MalAtom::Boolean(b) => write!(f, "{}", b),
            MalAtom::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for MalType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalType::Atom(v) => write!(f, "{}", v),
            MalType::List(l) => write!(
                f,
                "({})",
                l.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MalType::Vector(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MalType::Map(m) => write!(
                f,
                "{{{}}}",
                m.iter()
                    .map(|(k, v)| { format!("{} {}", k.to_string(), v.to_string()) })
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MalType::Function(func) => write!(f, "[function {}]", func.name,),
        }
    }
}

pub fn pr_str(code: &MalType) -> String {
    code.to_string()
}

impl fmt::Debug for MalFunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.debug_struct("Function")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalError::Normal(s) => write!(f, "{}", s),
            MalError::Parsing(s) => write!(f, "Parsing error{}", s),
            MalError::Resolve(s) => write!(f, "Symbol `{}` does not exist", s),
        }
    }
}
