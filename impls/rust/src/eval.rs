use std::collections::HashMap;

use crate::{
    env::Env,
    types::{MalAtom, MalError, MalType},
};

fn eval_ast(form: MalType, env: &Env) -> Result<MalType, MalError> {
    match form {
        MalType::Atom(MalAtom::Symbol(form1)) => env.get(&form1),
        MalType::List(l) => {
            let mut new_list = Vec::with_capacity(l.len());
            for inner_form in l.iter() {
                new_list.push(eval_form(inner_form.clone(), env)?);
            }
            Ok(MalType::List(new_list))
        }
        MalType::Vector(v) => {
            let mut new_vector = Vec::with_capacity(v.len());
            for inner_form in v.iter() {
                new_vector.push(eval_form(inner_form.clone(), env)?);
            }
            Ok(MalType::Vector(new_vector))
        }
        MalType::Map(m) => {
            let mut new_map = HashMap::with_capacity(m.len());
            for (key, val) in m.iter() {
                new_map.insert(key.clone(), eval_form(val.clone(), env)?);
            }
            Ok(MalType::Map(new_map))
        }
        _ => Ok(form.clone()),
    }
}

pub fn eval_form(form: MalType, env: &Env) -> Result<MalType, MalError> {
    match eval_ast(form, env)? {
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
    }
}
