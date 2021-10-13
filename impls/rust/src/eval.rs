use std::collections::HashMap;

use crate::{
    env::Env,
    types::{MalAtom, MalError, MalType},
};

fn resolve_ast(form: MalType, env: &mut Env) -> Result<MalType, MalError> {
    match form {
        MalType::Atom(MalAtom::Symbol(form1)) => env.get(&form1),
        MalType::List(l) => {
            let mut new_list = Vec::with_capacity(l.len());
            for inner_form in l {
                new_list.push(eval_form(inner_form.clone(), env)?);
            }
            Ok(MalType::List(new_list))
        }
        MalType::Vector(v) => {
            let mut new_vector = Vec::with_capacity(v.len());
            for inner_form in v {
                new_vector.push(eval_form(inner_form.clone(), env)?);
            }
            Ok(MalType::Vector(new_vector))
        }
        MalType::Map(m) => {
            let mut new_map = HashMap::with_capacity(m.len());
            for (key, val) in m {
                new_map.insert(key.clone(), eval_form(val.clone(), env)?);
            }
            Ok(MalType::Map(new_map))
        }
        _ => Ok(form.clone()),
    }
}

fn execute_def(args: Vec<MalType>, env: &mut Env) -> Result<MalType, MalError> {
    if args.len() != 2 {
        return Err(MalError::Normal(format!(
            "Wrong number of args for def!. Need 2, received {}",
            args.len(),
        )));
    }
    match (args.get(0), args.get(1)) {
        (Some(&MalType::Atom(MalAtom::Symbol(ref new_symbol))), Some(arg)) => {
            let evaled_arg = eval_form(arg.clone(), env)?;
            Ok(env.set(new_symbol.clone(), evaled_arg))
        }
        (Some(non_sym), None) => Err(MalError::Normal(format!(
            "First arg to def! must be a symbol. Given {:?}",
            non_sym,
        ))),
        _ => Err(MalError::Normal(String::from(
            "Second arg to def! must exist, given None",
        ))),
    }
}

fn execute_let_star(args: Vec<MalType>, env: &mut Env) -> Result<MalType, MalError> {
    if args.len() < 2 {
        return Err(MalError::Normal(format!(
            "Wrong number of args for let*. Need 2 or more, received {}",
            args.len(),
        )));
    }
    env.push_layer();
    match args.get(0) {
        None => unreachable!("First let* arg doesn't exist???"),
        Some(MalType::List(ref l)) | Some(MalType::Vector(ref l)) => {
            for pair in l {
                let p = if let MalType::List(p) | MalType::Vector(p) = pair {
                    p
                } else {
                    return Err(MalError::Normal(String::from("let* needs tuples")));
                };
                if p.len() == 2 {
                    match &p[0] {
                        MalType::Atom(MalAtom::Symbol(s)) => {
                            let evaled_rhs = eval_form(p[1].clone(), env)?;
                            env.set(s.clone(), evaled_rhs);
                        }
                        non_sym => {
                            return Err(MalError::Normal(format!(
                                "let* binding needs a symbol for lhs: {:?}",
                                non_sym
                            )));
                        }
                    }
                } else {
                    return Err(MalError::Normal(String::from("let* binding needs 2 parts")));
                }
            }
        }
        _ => {
            return Err(MalError::Normal(String::from(
                "Second arg to def! must exist, given None",
            )));
        }
    }
    let nested_forms = Vec::from(&args[1..]);
    let results: Vec<Result<MalType, MalError>> = nested_forms
        .into_iter()
        .map(|form| eval_form(form, env))
        .collect();
    env.pop_layer();

    if let Some(result) = results.last() {
        result.clone()
    } else {
        Ok(MalType::Atom(MalAtom::Nil))
    }
}

pub fn eval_form(form: MalType, env: &mut Env) -> Result<MalType, MalError> {
    match form {
        MalType::List(l) => match l.get(0) {
            None => Ok(MalType::List(l)),
            Some(MalType::Atom(MalAtom::Symbol(sym))) if sym.name == "def!" => {
                execute_def(Vec::from(&l[1..]), env)
            }
            Some(MalType::Atom(MalAtom::Symbol(sym))) if sym.name == "let*" => {
                execute_let_star(Vec::from(&l[1..]), env)
            }
            _ => match resolve_ast(MalType::List(l), env)? {
                MalType::List(evaled_list) => {
                    if evaled_list.is_empty() {
                        return Ok(MalType::List(evaled_list));
                    }
                    let symbol = evaled_list[0].clone();
                    let args = Vec::from(&evaled_list[1..]);
                    match symbol {
                        MalType::Function(func) => func.invoke(args),
                        _ => Err(MalError::Normal(format!(
                            "Symbol `{:?}` is not a function",
                            symbol
                        ))),
                    }
                }
                non_list => Ok(non_list),
            },
        },
        non_list => resolve_ast(non_list, env),
    }
}
