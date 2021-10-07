#![allow(non_snake_case)]

use mal::eval;
use mal::printer;
use mal::reader;
use mal::types::{MalEnv, MalError, MalType};
use rustyline::{error::ReadlineError, Editor};

fn READ(inp: &str) -> Result<Vec<MalType>, MalError> {
    reader::read_str(inp)
}

fn EVAL(form: &MalType, env: &MalEnv) -> Result<MalType, MalError> {
    eval::eval_form(form, env)
}

fn EVAL_forms(forms: Vec<MalType>, env: &MalEnv) -> Result<Vec<MalType>, MalError> {
    forms.iter().map(|form| EVAL(form, env)).collect()
}

fn PRINT(form: Vec<MalType>) -> Result<String, MalError> {
    Ok(form
        .iter()
        .map(printer::pr_str)
        .collect::<Vec<String>>()
        .join("\n"))
}

fn rep(inp: &str, env: &MalEnv) -> Result<String, MalError> {
    READ(inp)
        .and_then(|forms| EVAL_forms(forms, env))
        .and_then(PRINT)
}

pub fn prompt(env: MalEnv) {
    let mut rl = Editor::<()>::new();
    rl.load_history(reader::MAL_HISTORY).unwrap_or_default();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(reader::MAL_HISTORY).unwrap();
                match rep(line.as_str(), &env) {
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
    let env = MalEnv::new();
    prompt(env);
}

#[cfg(test)]
mod tests {
    use crate::rep;
    use mal::types::{MalEnv, MalError};
    use regex::Regex;
    use std::fs;

    #[test]
    fn mal_tests() {
        let env = MalEnv::new();
        let tests = fs::read_to_string("tests/step2_eval.mal")
            .expect("Something went wrong reading the file");
        for (idx, p) in tests
            .lines()
            .filter_map(|l| {
                if l.starts_with(";;") || l.starts_with(";>>>") || l.trim().is_empty() {
                    None
                } else {
                    Some(l.to_string())
                }
            })
            .collect::<Vec<String>>()
            .chunks(2)
            .enumerate()
        {
            let input = p[0].to_owned();

            if let Some(expected) = p.get(1) {
                if expected.starts_with(";=>") {
                    let stripped = expected.strip_prefix(";=>").unwrap_or(expected);
                    match rep(&input, &env) {
                        Ok(result) => {
                            assert!(
                                stripped == result,
                                "\nGiven    : `{}`\nExpected : `{}`\nGot      : `{}`",
                                input,
                                stripped,
                                result,
                            )
                        }
                        Err(err) => panic!("Got an unexpected error: {} for input: {}", err, input),
                    }
                } else if expected.starts_with(";/") || expected.starts_with(";.*") {
                    let replaced = expected.replace('{', "\\{");
                    let stripped = replaced.strip_prefix(";/").unwrap_or(&replaced);
                    match rep(&input, &env) {
                        Err(MalError::Parsing(result)) | Err(MalError::Resolve(result)) => {
                            let result_match = Regex::new(&format!("(?is){}", stripped))
                                .unwrap()
                                .is_match(&result.to_string());
                            assert!(
                                result_match,
                                "\nGiven    : `{}`\nExpected : `{}`\nGot      : `{}`",
                                input, stripped, result,
                            )
                        }
                        Err(err) => panic!("Got an unexpected error: {} for input: {}", err, input),
                        Ok(result) => panic!(
                            "Parsed when expected to fail: {} for input: {}",
                            result, input
                        ),
                    }
                } else {
                    panic!("unhandled {:?}", p);
                }
            } else {
                unreachable!("single-element line: {}, {:?}", 2 * idx, p);
            }
        }
    }
}
