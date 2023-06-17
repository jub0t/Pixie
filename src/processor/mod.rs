pub fn get_all_space_indexes(code: &str) -> Vec<usize> {
    let mut space_indexes: Vec<usize> = Vec::new();

    code.char_indices()
        .filter(|(_, character)| *character == ' ')
        .for_each(|(index, _)| space_indexes.push(index));

    space_indexes
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

pub fn track_until_nl(code: &str, start: usize) -> usize {
    if let Some(nl_index) = code[start..].find('\n') {
        return start + nl_index;
    }

    code.len()
}

pub fn split_by_nl(code: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in code.char_indices() {
        if c == '\n' {
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

pub fn get_next_space_index(index: usize, spaces: &[usize]) -> Option<usize> {
    for &space_index in spaces.iter().skip_while(|&i| *i <= index) {
        return Some(space_index);
    }

    None
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