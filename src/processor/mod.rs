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

pub fn track_until_nl_sc(code: &str, start: usize) -> usize {
    let mut index = start;
    let code_len = code.len();

    while index < code_len {
        let current_char = code.chars().nth(index).unwrap();

        if current_char == '\n' || current_char == ';' {
            break;
        }

        index += 1;
    }

    index
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

pub fn track_until_left_paren(code: &str) -> Option<&str> {
    let end_pos = code.find('(')?;

    Some(&code[..end_pos])
}

pub fn track_until_function_end_brackets(code: &str) -> &str {
    let start_idx = code.find("function").unwrap_or(0); // Find the index of the first occurrence of "fn" in the code
    let mut open_braces = 0;
    let mut end_idx = start_idx;

    for (idx, c) in code[start_idx..].char_indices() {
        if c == '{' {
            open_braces += 1;
        } else if c == '}' {
            open_braces -= 1;
            if open_braces == 0 {
                end_idx = idx + start_idx + 1; // Add 1 to include the closing brace in the tracked portion
                break;
            }
        }
    }

    &code[start_idx..end_idx]
}

pub fn is_alphanumeric_str(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_alphanumeric() || c.to_string() == "_")
}

pub fn split_until_first<'a>(code: &'a str, splitter: &'a str) -> (&'a str, &'a str) {
    if let Some(index) = code.find(splitter) {
        let (first_part, second_part) = code.split_at(index);
        (first_part, &second_part[splitter.len()..])
    } else {
        (code, "")
    }
}

pub fn get_paren_indexes(code: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    for (index, char) in code.chars().enumerate() {
        if char == '(' {
            stack.push(index);
        } else if char == ')' {
            if let Some(opening_index) = stack.pop() {
                result.push((opening_index, index));
            }
        }
    }

    result
}

pub fn process_parens(mut input: String) -> String {
    if let Some(first_char) = input.chars().next() {
        if first_char == '(' {
            input.remove(0);
        }
    }

    if let Some(last_char) = input.chars().last() {
        if last_char == ')' {
            input.pop();
        }
    }

    input
}

pub fn get_contents_within_func(funccode: &str) -> &str {
    if let Some(end_index) = funccode.rfind('}') {
        if let Some(start_index) = funccode.find('{') {
            let code = &funccode[start_index + 1..end_index];
            return code;
        }
    }

    return "";
}
