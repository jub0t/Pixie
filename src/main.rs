mod enums;
mod modules;
mod processor;

use modules::*;
use processor::*;
use std::{
    collections::HashMap,
    env::{self},
    fs,
};

fn main() {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as an argument.");
        return;
    }

    let file_name = args[1].clone();
    let bytes = fs::read(file_name);

    match bytes {
        Ok(bytes) => {
            let code = String::from_utf8(bytes).unwrap();
            let to_parse = split_by_nl(code.as_str());

            for exp in to_parse {
                let code = exp;
                let chars = code.clone();
                let chars = chars.split("").enumerate();
                let spaces = get_all_space_indexes(code);
                let mut stack = "".to_string();

                'chariter: for (index, character) in chars.clone() {
                    stack = stack.to_string() + character;

                    let kw = keyword_to_enum(stack.clone());
                    match kw {
                        Keywords::CONST | Keywords::LET => {
                            match get_next_space_index(index, &spaces) {
                                Some(next_space_index) => {
                                    let to = next_space_index - 1;
                                    let from = index + 1;
                                    let var = get_expression(&code, from, to).unwrap();

                                    if variable_name_valid(var) {
                                        let vf = from + var.len();
                                        let vt = to + var.len();

                                        if equal_sign_exist(&code, vf, vt) {
                                            let exind = track_until_nl(&code, vt) - 1;
                                            let varval = get_expression(&code, vt, exind)
                                                .unwrap()
                                                .replace("\"", "");
                                            let vtype = identify_type(&varval);
                                            let varval = parse_to_type(&varval, vtype.clone());

                                            let vkey = var.clone();
                                            let mutable: bool = match kw {
                                                Keywords::LET => true,
                                                Keywords::CONST => false,
                                                _ => false,
                                            };

                                            let data = Variable {
                                                vtype,
                                                mutable,
                                                value: Box::new(varval),
                                            };

                                            match variables.get(vkey) {
                                                Some(var) => {
                                                    if var.mutable {
                                                        variables.insert(vkey.to_string(), data);
                                                    }
                                                }
                                                None => {
                                                    variables.insert(vkey.to_string(), data);
                                                }
                                            }

                                            stack = "".to_string();
                                            continue 'chariter;
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
                            }
                        }
                        _ => {}
                    }

                    let method = method_to_enum(stack.clone());
                    let params = get_all_paren_indexes(code);

                    for (start, end) in params {
                        let to_print = get_all_param(code, start + 1, end - 1);

                        'paramiter: for val in to_print {
                            if is_literal(val) {
                                let lt = identify_type(val);
                                let lval = parse_to_type(val, lt.clone());

                                print_variable_value(&Variable {
                                    vtype: lt,
                                    value: lval,
                                    mutable: false,
                                });

                                match method {
                                    Methods::PRINT => {
                                        println!("");
                                    }
                                    _ => {}
                                }

                                break 'chariter;
                            } else {
                                match variables.get(val) {
                                    None => {
                                        println!("Variable '{}' is not defined", val)
                                    }
                                    Some(value) => match method {
                                        Methods::ECHO => {
                                            print_variable_value(value);
                                            break 'paramiter;
                                        }
                                        Methods::PRINT => {
                                            print_variable_value(value);
                                            println!();
                                            break 'paramiter;
                                        }
                                        _ => {}
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("Error reading the file: {}", err);
        }
    }
}
