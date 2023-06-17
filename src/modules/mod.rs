pub use crate::enums::*;
use std::any::Any;

pub fn keyword_to_enum(kw: String) -> Keywords {
    match kw.to_lowercase().as_str() {
        "const" => return Keywords::CONST,
        "let" => return Keywords::LET,
        _ => return Keywords::NONE,
    }
}

pub fn method_to_enum(kw: String) -> Methods {
    match kw.to_lowercase().as_str() {
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
        Type::String(_) => Box::new(Type::String(value.to_string())),
    }
}

pub fn print_variable_value(value: &Variable) {
    match &value.vtype {
        Type::I32(i) => print!("{}", i),
        Type::F64(f) => print!("{}", f),
        Type::Bool(b) => print!("{}", b),
        Type::String(s) => print!("{}", s),
    }
}
