use crate::types::{MalAtom, MalError, MalType};
use pest::{Parser, error, iterators::Pair};
use std::collections::HashMap;

pub static MAL_HISTORY: &str = ".mal-history";

#[derive(Parser)]
#[grammar = "parser/mal-grammer.pest"]
pub struct MalParser;

fn parse_atom(pair: Pair<Rule>) -> MalAtom {
    match pair.as_rule() {
        Rule::number => MalAtom::Integer(pair.as_str().parse().unwrap()),
        Rule::symbol => MalAtom::Symbol(pair.as_str().to_owned()),
        Rule::string => MalAtom::Str(pair.as_str().to_owned()),
        Rule::keyword => MalAtom::Keyword({
            let kw = pair.as_str();
            kw.strip_prefix(':').unwrap_or(kw).to_owned()
        }),
        Rule::boolean => MalAtom::Boolean(pair.as_str() == "true"),
        Rule::nil => MalAtom::Nil,
        _ => unreachable!("atom? {:?}", pair.as_rule()),
    }
}

fn parse_value(pair: Pair<Rule>) -> Option<MalType> {
    match pair.as_rule() {
        Rule::list => Some(MalType::List(
            pair.into_inner().filter_map(parse_value).collect(),
        )),
        Rule::vector => Some(MalType::Vector(
            pair.into_inner().filter_map(parse_value).collect(),
        )),
        Rule::map => Some(MalType::Map({
            let mut hm = HashMap::new();
            for p in pair.into_inner().collect::<Vec<Pair<Rule>>>().chunks(2) {
                let k = parse_atom(p[0].clone());
                let v = parse_value(p[1].clone());
                if let Some(v) = v {
                    hm.insert(k, v);
                }
            }
            hm
        })),
        Rule::number
        | Rule::symbol
        | Rule::string
        | Rule::keyword
        | Rule::boolean
        | Rule::nil => Some(MalType::Atom(parse_atom(pair))),
        _ => unreachable!("value? {:?}", pair.as_rule()),
    }
}

fn parse_error(err: error::Error<Rule>) -> MalError {
    let err_str = err.to_string();
    MalError::Parsing(format!(
        "Parsing error{}{}",
        if (err_str.matches('(').count() != err_str.matches(')').count())
            || (err_str.matches('[').count() != err_str.matches(']').count())
            || ((err_str.matches('"').count() % 2) != 0)
            || ((err_str.matches('\\').count() % 2) != 0)
        {
            " (EOF)"
        } else {
            ""
        },
        err,
    ))
}

pub type ParseResult = Result<Vec<MalType>, MalError>;

pub fn read_str(input: &str) -> ParseResult {
    match MalParser::parse(Rule::values, input) {
        Ok(pairs) => Ok(pairs
            .filter_map(|p| {
                if p.as_rule() == Rule::EOI {
                    None
                } else {
                    parse_value(p)
                }
            })
            .collect()),
        Err(error) => Err(parse_error(error)),
    }
}
