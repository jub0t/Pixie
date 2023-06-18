mod enums;
mod modules;
mod parser;
mod processor;
mod tokenize;

use modules::*;
use processor::*;
use std::{
    collections::HashMap,
    env::{self},
    fs,
};

use crate::{parser::*, tokenize::*};

fn main() {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let mut functions: HashMap<String, Function> = HashMap::new();

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
            let to_parse = tokenize_code(code.as_str());

            'lineiter: for (line_ind, line) in to_parse.iter().enumerate() {
                let code = line.as_str();
                let chars = code.clone().split("").enumerate();
                let spaces = get_all_space_indexes(code);
                let mut stack = "".to_string();

                'chariter: for (index, character) in chars.clone() {
                    stack = stack.to_string() + character;

                    let kw = keyword_to_enum(stack.clone());
                    match kw {
                        Keywords::FUNCTION => {
                            if let Some(chracter) = code.chars().nth(index) {
                                if chracter.to_string() != " " {
                                    continue 'lineiter;
                                }
                            }

                            // Parse Function
                            let splitted = split_until_first(code, " ");
                            let fn_core = splitted.1;
                            let fn_name = track_until_left_paren(fn_core);
                            let param_list = get_paren_indexes(fn_core)[0];
                            let args = fn_core[param_list.0 + 1..param_list.1]
                                .split(",")
                                .map(|param| param.trim())
                                .collect::<Vec<&str>>();

                            match fn_name {
                                Some(name) => {
                                    let params = parse_params(args.clone());

                                    functions.insert(
                                        name.to_string(),
                                        Function {
                                            action: vec![],
                                            ftype: FuntionType::CUSTOM,
                                            name: name.to_string(),
                                            params,
                                        },
                                    );
                                }
                                _ => {
                                    println!("Function name {:?} is Invalid", fn_name)
                                }
                            }

                            continue 'chariter;
                        }
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

                                            stack = String::new();
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

                    // Built-in method parser
                    let method = method_to_enum(stack.clone());
                    let params = get_all_paren_indexes(code);
                    match method {
                        Methods::PRINT | Methods::ECHO => {
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
                        _ => {}
                    }
                }

                // Custom function is being called
                let parens = get_paren_indexes(stack.as_str());
                if parens.len() > 0 {
                    let pos = parens[0];
                    let func_name = &stack[0..pos.0];

                    if is_alphanumeric_str(func_name) && func_name.len() > 0 {
                        match method_to_enum(func_name.to_string()) {
                            Methods::PRINT | Methods::ECHO => {}
                            _ => {
                                if let Some(function) = functions.get(func_name) {
                                    let args_string = stack.replace(func_name, "");
                                    let mut args: HashMap<String, Argument> = HashMap::new();

                                    for (index, arg) in
                                        process_parens(args_string).split(",").enumerate()
                                    {
                                        let mut arg_type = identify_type(arg);
                                        let mut parsed_val = parse_to_type(arg, arg_type.clone());

                                        match arg_type {
                                            Type::String(_) => {
                                                let nq = remove_quotes_from_sides(arg);

                                                arg_type = identify_type(nq.as_str());
                                                parsed_val = Box::new(nq)
                                            }
                                            _ => {}
                                        }

                                        if let Some(paraminfo) = &function.params.get(index) {
                                            args.insert(
                                                paraminfo.value.clone(),
                                                Argument {
                                                    ptype: arg_type,
                                                    value: parsed_val,
                                                },
                                            );
                                        } else {
                                            println!(
                                                "Extra parameter(s) passed while calling function {:?}",
                                                func_name
                                            );
                                        }
                                    }
                                } else {
                                    println!("Function {:?} is never defined", func_name)
                                }
                            }
                        }

                        continue 'lineiter;
                    }
                }
            }
        }
        Err(err) => {
            println!("Error reading the file: {}", err);
        }
    }
}
