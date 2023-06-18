use crate::*;
use std::ops::Add;

pub fn tokenize_code(code: &str) -> Vec<String> {
    let mut stack = String::new();
    let mut lines = vec![];

    for (index, character) in code.split("").enumerate() {
        stack = stack.add(character);

        let kw = keyword_to_enum(stack.clone());
        let md = method_to_enum(stack.clone());

        match md {
            Methods::NONE => {}
            _ => {
                let parsed = track_until_nl_sc(code, index);
                let code = stack.clone() + &code[index..parsed];
                lines.push(code);
            }
        }

        match kw {
            Keywords::FUNCTION => {
                let code = track_until_function_end_brackets(code);
                lines.push(code.to_string());

                continue;
            }

            Keywords::NONE => {}
            _ => {
                let parsed = track_until_nl_sc(code, index);
                let code = stack.clone() + &code[index..parsed];
                let code = code.as_str();

                lines.push(code.to_string());

                continue;
            }
        }

        if character == "\r" || character == "\n" || character == ";" {
            stack = String::new();
        }
    }

    return lines;
}
