use crate::types::{MalAtom, MalType};

fn pr_str_atom(code: &MalAtom) -> String {
    match code {
        MalAtom::Symbol(s) => s.to_string(),
        MalAtom::Integer(i) => i.to_string(),
        MalAtom::Str(s) => s.to_string(),
        MalAtom::Keyword(k) => format!(":{}", k),
        MalAtom::Boolean(b) => b.to_string(),
        MalAtom::Nil => "nil".to_string(),
    }
}

pub fn pr_str(code: &MalType) -> String {
    match code {
        MalType::List(l) => format!(
            "({})",
            l.iter().map(pr_str).collect::<Vec<String>>().join(" ")
        ),
        MalType::Vector(v) => format!(
            "[{}]",
            v.iter().map(pr_str).collect::<Vec<String>>().join(" ")
        ),
        MalType::Atom(v) => pr_str_atom(v),
        MalType::Map(m) => format!(
            "{{{}}}",
            m.iter()
                .map(|(k, v)| { format!("{} {}", pr_str_atom(k), pr_str(v)) })
                .collect::<Vec<String>>()
                .join(" ")
        ),
    }
}
