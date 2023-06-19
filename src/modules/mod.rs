pub use crate::enums::*;
use std::{any::Any, collections::HashMap, i8};

pub fn keyword_to_enum(kw: String) -> Keywords {
    match kw.to_lowercase().as_str() {
        "function" => return Keywords::FUNCTION,
        "const" => return Keywords::CONST,
        "let" => return Keywords::LET,
        _ => return Keywords::NONE,
    }
}

pub fn method_to_enum(md: String) -> Methods {
    match md.to_lowercase().as_str() {
        "echo" => return Methods::ECHO,
        "print" => return Methods::PRINT,
        _ => return Methods::NONE,
    }
}

pub fn identify_type(value: &str) -> Type {
    let trimmed_value = value.trim();

    if let Ok(parsed_value) = trimmed_value.parse::<i32>() {
        Type::I32(parsed_value)
    } else if let Ok(parsed_value) = trimmed_value.parse::<f64>() {
        Type::F64(parsed_value)
    } else if let Ok(parsed_value) = trimmed_value.parse::<bool>() {
        Type::Bool(parsed_value)
    } else {
        Type::String(trimmed_value.to_string())
    }
}

pub fn parse_to_type(value: &str, vtype: Type) -> Box<dyn Any> {
    let numval = value.replace(" ", "");
    match vtype {
        Type::I32(_) => {
            if let Ok(parsed_value) = numval.parse::<i32>() {
                Box::new(Type::I32(parsed_value))
            } else {
                Box::new(Type::String(value.to_string()))
            }
        }
        Type::F64(_) => {
            if let Ok(parsed_value) = numval.parse::<f64>() {
                Box::new(Type::F64(parsed_value))
            } else {
                Box::new(Type::String(value.to_string()))
            }
        }
        Type::Bool(_) => {
            if let Ok(parsed_value) = value.parse::<bool>() {
                Box::new(Type::Bool(parsed_value))
            } else {
                Box::new(Type::String(value.to_string()))
            }
        }
        Type::String(_) => Box::new(Type::String(value.to_string().replace("\"", ""))),
    }
}

pub fn print_variable_value(value: &Variable) {
    match &value.vtype {
        Type::I32(i) => print!("{}", i),
        Type::F64(f) => print!("{}", f),
        Type::Bool(b) => print!("{}", b),
        Type::String(s) => {
            let nq = remove_quotes_from_sides(s);
            let reformed = nq.replace("\\n", "\n");

            print!("{}", reformed)
        }
    }
}

pub fn is_literal(param: &str) -> bool {
    if param.starts_with('"') && param.ends_with('"') {
        true
    } else if let Ok(_) = param.parse::<i32>() {
        true
    } else if let Ok(_) = param.parse::<f64>() {
        true
    } else {
        false
    }
}

pub fn remove_quotes_from_sides(code: &str) -> String {
    if code.len() >= 2 && code.starts_with('"') && code.ends_with('"') {
        code[1..code.len() - 1].to_string()
    } else {
        code.to_string()
    }
}

pub fn merge_params_with_variables(params: Vec<Param>, vars: &mut HashMap<String, Variable>) {
    for p in params {
        vars.insert(
            p.name,
            Variable {
                vtype: p.ptype,
                mutable: true,
                value: Box::new(p.value),
            },
        );
    }
}

pub fn params_to_hashmap(params: Vec<Param>) -> HashMap<String, Param> {
    let mut map: HashMap<String, Param> = HashMap::new();

    for (index, param) in params.iter().enumerate() {
        map.insert(
            param.name.clone(),
            Param {
                index: index as i8,
                name: param.to_owned().name,
                ptype: param.to_owned().ptype,
                value: param.to_owned().value,
            },
        );
    }

    return map;
}
