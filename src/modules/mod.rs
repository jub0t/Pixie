use std::any::Any;

pub enum Keywords {
    NONE = 0,
    CONST = 1,
}

#[derive(PartialEq)]
pub enum Methods {
    NONE = 0,
    PRINT = 1,
    PRINTLN = 2,
}

pub struct Variable {
    pub vtype: Type,
    pub value: Box<dyn Any>,
}

#[derive(Debug, Clone)]
pub enum Type {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
}

pub fn get_all_space_indexes(code: &str) -> Vec<usize> {
    let mut space_indexes: Vec<usize> = Vec::new();

    for (index, character) in code.char_indices() {
        if character == ' ' {
            space_indexes.push(index);
        }
    }

    space_indexes
}

pub fn get_all_paren_indexes(code: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    for (i, c) in code.char_indices() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            if let Some(open_index) = stack.pop() {
                result.push((open_index, i));
            }
        }
    }

    result
}

pub fn keyword_to_enum(kw: String) -> Keywords {
    match kw.to_lowercase().as_str() {
        "const" => return Keywords::CONST,
        _ => return Keywords::NONE,
    }
}

pub fn method_to_enum(kw: String) -> Methods {
    match kw.to_lowercase().as_str() {
        "lnprint" => return Methods::PRINTLN,
        "print" => return Methods::PRINT,
        _ => return Methods::NONE,
    }
}

pub fn get_next_space_index(index: usize, spaces: &[usize]) -> Option<usize> {
    for &space_index in spaces.iter().skip_while(|&i| *i <= index) {
        return Some(space_index);
    }

    None
}

pub fn variable_name_valid(variable_name: &str) -> bool {
    if variable_name.is_empty() {
        println!("EMpty var");
        return false;
    }

    if !variable_name.chars().next().unwrap().is_alphabetic() {
        println!("non aplha var");
        return false;
    }

    for c in variable_name.chars() {
        if !c.is_alphanumeric() && c.to_string() != "_" {
            println!("Variable {} is Non alpha-numerical", c);
            return false;
        }
    }

    true
}

pub fn equal_sign_exist(code: &str, from: usize, to: usize) -> bool {
    if from > to || to >= code.len() {
        return false;
    }

    for c in code[from..=to].chars() {
        if c == '=' {
            return true;
        }
    }

    false
}

pub fn get_all_param(code: &str, from: usize, to: usize) -> Vec<&str> {
    if from > to || to >= code.len() {
        return Vec::new();
    }

    let code_slice = &code[from..=to];

    let params: Vec<&str> = code_slice.split(',').map(|param| param.trim()).collect();

    params
}

pub fn get_expression(code: &str, from: usize, to: usize) -> Option<&str> {
    if from > to || to >= code.len() {
        return None;
    }

    Some(&code[from..=to])
}

pub fn track_until_nl_or_sm(code: &str, start: usize) -> usize {
    if let Some(sm_index) = code[start..].find(';') {
        return start + sm_index;
    }

    if let Some(nl_index) = code[start..].find('\n') {
        return start + nl_index;
    }

    code.len()
}

pub fn split_by_sm_nl(code: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in code.char_indices() {
        if c == ';' || c == '\n' {
            if start != i {
                result.push(&code[start..i]);
            }
            start = i + 1;
        }
    }

    if start < code.len() {
        result.push(&code[start..]);
    }

    result
}

pub fn identify_type(value: &str) -> Type {
    if let Ok(parsed_value) = value.replace(" ", "").parse::<i32>() {
        Type::I32(parsed_value)
    } else if let Ok(parsed_value) = value.replace(" ", "").parse::<f64>() {
        Type::F64(parsed_value)
    } else if let Ok(parsed_value) = value.parse::<bool>() {
        Type::Bool(parsed_value)
    } else {
        Type::String(value.to_string())
    }
}

pub fn parse_to_type(value: &str, vtype: Type) -> Box<dyn Any> {
    match vtype {
        Type::I32(_) => Box::new(value.parse::<i32>().ok().map(Type::I32)),
        Type::F64(_) => Box::new(value.parse::<f64>().ok().map(Type::F64)),
        Type::Bool(_) => Box::new(value.parse::<bool>().ok().map(Type::Bool)),
        Type::String(_) => Box::new(Type::String(value.to_string())),
    }
}

pub fn print(var: &Variable) {
    match &var.vtype {
        Type::I32(value) => print!("{}", value),
        Type::F64(value) => print!("{}", value),
        Type::Bool(value) => print!("{}", value),
        Type::String(value) => print!("{}", value),
    }
}
