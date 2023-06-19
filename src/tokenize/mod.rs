use crate::*;
use std::ops::Add;

pub fn tokenize_code(code: &str) -> Vec<String> {
    let mut stack = String::new();
    let mut lines = vec![];

    for (index, character) in code.split("").enumerate() {
        if character == " " {
            continue;
        }

        stack = stack.add(character);

        let kw = keyword_to_enum(stack.clone());
        let md = method_to_enum(stack.clone());

        match md {
            Methods::NONE => {}
            _ => {
                let parsed = track_until_nl_sc(code, index);
                let code = stack.clone() + &code[index..parsed];
                lines.push(code);
                stack = String::new();
            }
        }

        match kw {
            Keywords::FUNCTION => {
                let code = track_until_function_end_brackets(code);
                lines.push(code.to_string());
                stack = String::new();

                continue;
            }

            Keywords::NONE => {}
            _ => {
                let parsed = track_until_nl_sc(code, index);
                let code = stack.clone() + &code[index..parsed];
                let code = code.as_str();

                lines.push(code.to_string());
                stack = String::new();

                continue;
            }
        }

        // Custom function is being called
        let parens = get_paren_indexes(stack.as_str());
        if parens.len() > 0 {
            let pos = parens[0];
            let func_name = &stack[0..pos.0];

            if is_alphanumeric_str(func_name) && func_name.len() > 0 {
                lines.push(stack.to_string());
                stack = String::new();
                continue;
            }
        }

        if character == "\r" || character == "\n" || character == ";" {
            stack = String::new();
        }
    }

    return lines;
}
