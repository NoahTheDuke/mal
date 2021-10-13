#![allow(non_snake_case)]

use mal::built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn};
use mal::env::Env;
use mal::eval;
use mal::printer;
use mal::reader;
use mal::types::{MalError, MalType};
use rustyline::{error::ReadlineError, Editor};

fn READ(inp: &str) -> Result<Vec<MalType>, MalError> {
    reader::read_str(inp)
}

fn EVAL(form: MalType, env: &mut Env) -> Result<MalType, MalError> {
    eval::eval_form(form, env)
}

fn EVAL_forms(forms: Vec<MalType>, env: &mut Env) -> Result<Vec<MalType>, MalError> {
    forms.into_iter().map(|form| EVAL(form, env)).collect()
}

fn PRINT(form: Vec<MalType>) -> Result<String, MalError> {
    Ok(form
        .iter()
        .map(printer::pr_str)
        .collect::<Vec<String>>()
        .join("\n"))
}

fn rep(inp: &str, env: &mut Env) -> Result<String, MalError> {
    READ(inp)
        .and_then(|forms| EVAL_forms(forms, env))
        .and_then(PRINT)
}

pub fn prompt(mut env: Env) {
    let mut rl = Editor::<()>::new();
    rl.load_history(reader::MAL_HISTORY).unwrap_or_default();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(reader::MAL_HISTORY).unwrap();
                match rep(line.as_str(), &mut env) {
                    Ok(result) => println!("{}", result),
                    Err(err) => println!("{}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(reader::MAL_HISTORY).unwrap();
}

fn main() {
    let mut env = Env::new();
    let plus = plus_fn();
    env.set(plus.name.clone(), MalType::Function(plus));
    let minus = minus_fn();
    env.set(minus.name.clone(), MalType::Function(minus));
    let multiply = multiply_fn();
    env.set(multiply.name.clone(), MalType::Function(multiply));
    let divide = divide_fn();
    env.set(divide.name.clone(), MalType::Function(divide));
    prompt(env);
}

#[cfg(test)]
mod tests {
    use crate::rep;
    use mal::built_ins::{divide_fn, minus_fn, multiply_fn, plus_fn};
    use mal::types::MalType;
    use mal::{env::Env, types::MalError};
    use regex::Regex;
    use std::fs;

    #[derive(Clone, Debug)]
    struct TestInput {
        inputs: Vec<String>,
        output: String,
    }

    #[test]
    fn mal_tests() {
        let mut env = Env::new();
        let plus = plus_fn();
        env.set(plus.name.clone(), MalType::Function(plus));
        let minus = minus_fn();
        env.set(minus.name.clone(), MalType::Function(minus));
        let multiply = multiply_fn();
        env.set(multiply.name.clone(), MalType::Function(multiply));
        let divide = divide_fn();
        env.set(divide.name.clone(), MalType::Function(divide));

        let tests = fs::read_to_string("tests/step3_env.mal")
            .expect("Something went wrong reading the file");

        let mut cleaned_tests = tests
            .lines()
            .filter_map(|l| {
                if l.starts_with(";;") || l.starts_with(";>>>") || l.trim().is_empty() {
                    None
                } else {
                    Some(l.to_string())
                }
            })
            .collect::<Vec<String>>()
            .into_iter();

        let mut test_cases = Vec::new();

        'outer: loop {
            let mut test = TestInput {
                inputs: Vec::new(),
                output: String::default(),
            };
            for next_input in &mut cleaned_tests {
                if next_input.starts_with(";=>")
                    || next_input.starts_with(";/")
                    || next_input.starts_with(";.*")
                {
                    test.output = next_input.clone();
                    test_cases.push(test);
                    continue 'outer;
                } else {
                    test.inputs.push(next_input.clone());
                }
            }
            break;
        }

        for t in test_cases {
            let expected = &t.output;

            if expected.starts_with(";=>") {
                let expected = expected.strip_prefix(";=>").unwrap();
                let results: Vec<Result<String, MalError>> =
                    t.inputs.iter().map(|input| rep(input, &mut env)).collect();
                match results.last() {
                    Some(Ok(result)) => {
                        assert!(
                            expected == result,
                            "\nGiven    : `{:?}`\nExpected : `{}`\nGot      : `{}`",
                            t.inputs,
                            expected,
                            result,
                        )
                    }
                    Some(Err(err)) => panic!("Got an unexpected error: {} for input: {:?}", err, t.inputs),
                    None => unreachable!("How did we get here?"),
                }
            } else if expected.starts_with(";/") || expected.starts_with(";.*") {
                let expected = expected.replace('{', "\\{");
                let expected = expected.strip_prefix(";/").unwrap();
                let results: Vec<Result<String, MalError>> =
                    t.inputs.iter().map(|input| rep(input, &mut env)).collect();
                match results.last() {
                    Some(Err(result @ MalError::Parsing(_))) | Some(Err(result @ MalError::Resolve(_))) => {
                        let result_match = Regex::new(&format!("(?is){}", expected))
                            .unwrap()
                            .is_match(&result.to_string());
                        assert!(
                            result_match,
                            "\nGiven    : `{:?}`\nExpected : `{}`\nGot      : `{}`",
                            t.inputs, expected, result,
                        )
                    }
                    Some(Err(err)) => panic!("Got an unexpected error: {} for input: {:?}", err, t.inputs),
                    Some(Ok(result)) => panic!(
                        "Parsed when expected to fail: {:?} for input: {:?}",
                        result, t.inputs
                    ),
                    None => unreachable!("wtf"),
                }
            } else {
                panic!("unhandled {:?}", t);
            }
        }
    }
}
