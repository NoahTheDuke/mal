use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Keyword {
    pub name: String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::keyword::Keyword;

    #[test]
    fn test() {
        assert_eq!(":a", Keyword { name: String::from("a") }.to_string());
    }
}
