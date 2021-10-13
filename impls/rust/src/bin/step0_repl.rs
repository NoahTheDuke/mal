#![allow(non_snake_case)]

use mal::reader;
use rustyline::{error::ReadlineError, Editor};

fn READ(inp: &str) -> &str {
    inp
}

fn EVAL(inp: &str) -> &str {
    inp
}

fn PRINT(inp: &str) -> String {
    inp.to_string()
}

fn rep(inp: &str) -> Result<String, String> {
    Ok(PRINT(EVAL(READ(inp))))
}

pub fn prompt() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(reader::MAL_HISTORY).unwrap();
                match rep(line.as_str()) {
                    Ok(result) => println!("{}", result),
                    Err(_) => unreachable!(),
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
    prompt();
}

#[cfg(test)]
mod tests {
    use crate::rep;
    use regex::Regex;
    use std::fs;

    #[test]
    fn mal_tests() {
        let tests = fs::read_to_string("tests/step0_repl.mal")
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
            let input = p[0].to_string();
            match rep(&input) {
                Ok(result) => {
                    if let Some(expected) = p.get(1) {
                        if expected.starts_with(";=>") {
                            let stripped = expected.strip_prefix(";=>").unwrap_or(expected);
                            assert!(
                                stripped == result,
                                "\nGiven    : `{}`\nExpected : `{}`\nGot      : `{}`",
                                input,
                                stripped,
                                result,
                            )
                        } else if expected.starts_with(";/") || expected.starts_with(";.*") {
                            let replaced = expected.replace('{', "\\{");
                            let stripped = replaced.strip_prefix(";/").unwrap_or(&replaced);
                            assert!(
                                Regex::new(&format!("(?is){}", stripped))
                                    .unwrap()
                                    .is_match(&result),
                                "\nGiven    : `{}`\nExpected : `{}`\nGot      : `{}`",
                                input,
                                stripped,
                                result,
                            )
                        } else {
                            panic!("unhandled {:?}", p);
                        }
                    } else {
                        unreachable!("single-element line: {}, {:?}", 2 * idx, p);
                    }
                }
                Err(err) => {
                    panic!("Normal err {}", err)
                }
            }
        }
    }
}
