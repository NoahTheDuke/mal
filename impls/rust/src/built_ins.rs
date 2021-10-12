use crate::{symbol::Symbol, types::{MalAtom, MalError, MalFunction, MalType}};

fn parse_numbers(args: Vec<MalType>) -> Result<Vec<isize>, MalError> {
    let mut new_args = Vec::with_capacity(args.len());
    for arg in args.iter() {
        if let MalType::Atom(MalAtom::Integer(num)) = arg {
            new_args.push(*num);
        } else {
            return Err(MalError::Normal(format!("`{:?}` is not a number", arg)));
        }
    }
    Ok(new_args)
}

fn plus(args: Vec<MalType>) -> Result<MalType, MalError> {
    let parsed_numbers = parse_numbers(args)?;
    match parsed_numbers.len() {
        0 => Ok(MalType::Atom(MalAtom::Integer(0))),
        1 => Ok(MalType::Atom(MalAtom::Integer(parsed_numbers[0]))),
        _ => Ok(MalType::Atom(MalAtom::Integer(parsed_numbers.iter().sum()))),
    }
}

pub fn plus_fn() -> MalFunction {
    MalFunction::new(Symbol::new("+"), plus)
}

fn minus(args: Vec<MalType>) -> Result<MalType, MalError> {
    let parsed_numbers = parse_numbers(args)?;
    match parsed_numbers.len() {
        0 => Ok(MalType::Atom(MalAtom::Integer(0))),
        1 => Ok(MalType::Atom(MalAtom::Integer(-parsed_numbers[0]))),
        _ => {
            let base = parsed_numbers[0];
            let rest: isize = parsed_numbers[1..].iter().sum();
            Ok(MalType::Atom(MalAtom::Integer(base - rest)))
        }
    }
}

pub fn minus_fn() -> MalFunction {
    MalFunction::new(Symbol::new("-"), minus)
}

fn multiply(args: Vec<MalType>) -> Result<MalType, MalError> {
    let parsed_numbers = parse_numbers(args)?;
    match parsed_numbers.len() {
        0 => Ok(MalType::Atom(MalAtom::Integer(1))),
        1 => Ok(MalType::Atom(MalAtom::Integer(parsed_numbers[0]))),
        _ => Ok(MalType::Atom(MalAtom::Integer(
            parsed_numbers.iter().product(),
        ))),
    }
}

pub fn multiply_fn() -> MalFunction {
    MalFunction::new(Symbol::new("*"), multiply)
}

fn divide(args: Vec<MalType>) -> Result<MalType, MalError> {
    let parsed_numbers = parse_numbers(args)?;
    match parsed_numbers.len() {
        0 => Err(MalError::Normal("/ requires at least 1 arg".to_string())),
        1 => Ok(MalType::Atom(MalAtom::Integer(1 / parsed_numbers[0]))),
        _ => Ok(MalType::Atom(MalAtom::Integer({
            let base = parsed_numbers[0];
            parsed_numbers[1..].iter().fold(base, |acc, cur| acc / cur)
        }))),
    }
}

pub fn divide_fn() -> MalFunction {
    MalFunction::new(Symbol::new("/"), divide)
}
