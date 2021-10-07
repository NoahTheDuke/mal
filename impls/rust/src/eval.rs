use std::collections::HashMap;

use crate::types::{MalAtom, MalEnv, MalError, MalType};

fn eval_ast(form: &MalType, env: &MalEnv) -> Result<MalType, MalError> {
    match form {
        MalType::Atom(MalAtom::Symbol(form1)) => {
            if let Some(op) = env.get(form1) {
                Ok(op.clone())
            } else {
                Err(MalError::Resolve(form1.clone()))
            }
        }
        MalType::List(l) => {
            let mut new_list = Vec::with_capacity(l.len());
            for inner_form in l.iter() {
                new_list.push(eval_form(inner_form, env)?);
            }
            Ok(MalType::List(new_list))
        }
        MalType::Vector(v) => {
            let mut new_vector = Vec::with_capacity(v.len());
            for inner_form in v.iter() {
                new_vector.push(eval_form(inner_form, env)?);
            }
            Ok(MalType::Vector(new_vector))
        }
        MalType::Map(m) => {
            let mut new_map = HashMap::with_capacity(m.len());
            for (key, val) in m.iter() {
                new_map.insert(key.clone(), eval_form(val, env)?);
            }
            Ok(MalType::Map(new_map))
        }
        _ => Ok(form.clone()),
    }
}

pub fn eval_form(form: &MalType, env: &MalEnv) -> Result<MalType, MalError> {
    match eval_ast(form, env)? {
        MalType::List(evaled_list) => {
            if evaled_list.is_empty() {
                return Ok(form.clone());
            }
            let symbol = &evaled_list[0];
            let args = &evaled_list[1..];
            match symbol {
                MalType::Function(f) => f.invoke(args),
                _ => Err(MalError::Normal(format!(
                    "Symbol `{:?}` is not a function",
                    symbol
                ))),
            }
        }
        non_list => Ok(non_list),
    }
}
