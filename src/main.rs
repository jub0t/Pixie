mod modules;

use modules::*;
use std::{collections::HashMap, env, fs};

fn main() {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let args: Vec<String> = env::args().collect();

    let file_name = args[1].clone();
    let bytes = fs::read(file_name);

    match bytes {
        Ok(bytes) => {
            let code = String::from_utf8(bytes).unwrap();

            let to_parse = split_by_sm_nl(code.as_str());

            for exp in to_parse {
                let code = exp;
                let chars = code.clone();
                let chars = chars.split("").enumerate();
                let spaces = get_all_space_indexes(code);
                let mut stack = "".to_string();

                // Parser
                for (index, char) in chars.to_owned() {
                    stack = stack.to_string() + char;

                    match keyword_to_enum(stack.clone()) {
                        Keywords::CONST => match get_next_space_index(index, &spaces) {
                            Some(next_space_index) => {
                                let to = next_space_index - 1;
                                let from = index + 1;
                                let var = get_expression(&code, from, to).unwrap();

                                if variable_name_valid(var) {
                                    let vf = from + var.len();
                                    let vt = to + var.len();

                                    if equal_sign_exist(&code, vf, vt) {
                                        let exind = track_until_nl_or_sm(&code, vt) - 1;
                                        let varval = get_expression(&code, vt, exind)
                                            .unwrap()
                                            .replace("\"", "");
                                        let vtype = identify_type(&varval);
                                        let varval = parse_to_type(&varval, vtype.clone());

                                        // DEFINE VARIABLE
                                        variables.insert(
                                            var.clone().to_string(),
                                            Variable {
                                                vtype,
                                                value: Box::new(varval),
                                            },
                                        );

                                        stack = "".to_string();
                                        continue;
                                    } else {
                                        println!(
                                            "Expected Equal Operator Within Column {}-{}",
                                            vf, vt
                                        );
                                    }
                                } else {
                                    println!("The variable name is not valid.");
                                }
                            }
                            None => {
                                println!("No space index found after {}", index);
                            }
                        },
                        _ => {}
                    }

                    let method = method_to_enum(stack.clone());
                    match method {
                        Methods::PRINT | Methods::PRINTLN => {
                            let params = get_all_paren_indexes(code);

                            for (start, end) in params {
                                let to_print = get_all_param(code, start + 1, end - 1);
                                for val in to_print {
                                    match variables.get(val) {
                                        None => {
                                            println!("Undefined Variable: {}", val);
                                        }
                                        Some(value) => {
                                            if method == Methods::PRINT {
                                                print(value);
                                            } else if method == Methods::PRINTLN {
                                                print(value);
                                                print!("\n")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    // println!("{:?}", newlines);
    // println!("{:?}", qoutes);
    // println!("{:?}", spaces);
}
