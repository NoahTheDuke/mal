use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol::Symbol;

    #[test]
    fn test() {
        assert_eq!(
            "a",
            Symbol {
                name: String::from("a")
            }
            .to_string()
        );
    }
}
