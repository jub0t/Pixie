use crate::*;

// pstr: ["t1: String", " t2: I32"]
pub fn parse_params(pstr: Vec<&str>) -> Vec<Param> {
    let mut params = vec![];

    for param_str in pstr {
        let tokens: Vec<&str> = param_str.split(':').map(|s| s.trim()).collect();
        let name = tokens[0];
        let ptype_str = tokens[1];
        let ptype = identify_type(ptype_str);

        params.push(Param {
            ptype,
            value: name.to_string(),
        })
    }

    return params;
}

pub fn parse_function(code: &str, line: String, functions: &mut HashMap<String, Function>) {
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

            // Parse Function Inside
            let func_code = get_contents_within_func(line.as_str());
            println!("{}", func_code);
        }
        _ => {
            println!("Function name {:?} is Invalid", fn_name)
        }
    }
}

pub fn parse_builtin_method(stack: String, code: &str, variables: &HashMap<String, Variable>) {
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

                        break 'paramiter;
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

pub fn parse_custom_function(stack: String, functions: &HashMap<String, Function>) {
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

                        for (index, arg) in process_parens(args_string).split(",").enumerate() {
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
                                    "Extra parameter passed while calling function {:?}",
                                    func_name
                                );
                            }
                        }
                    } else {
                        println!("Function {:?} is never defined", func_name)
                    }
                }
            }
        }
    }
}

pub fn parse_keyword(
    kw: Keywords,
    code: &str,
    index: usize,
    line: &str,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, Variable>,
    stack: &mut String,
    spaces: &[usize],
) {
    match kw {
        Keywords::FUNCTION => {
            if let Some(character) = code.chars().nth(index) {
                if character.to_string() != " " {
                    return;
                }
            }

            parse_function(code, line.to_owned(), functions);

            return;
        }
        Keywords::CONST | Keywords::LET => match get_next_space_index(index, spaces) {
            Some(next_space_index) => {
                let to = next_space_index - 1;
                let from = index + 1;
                let var = get_expression(&code, from, to).unwrap();

                if variable_name_valid(var) {
                    let vf = from + var.len();
                    let vt = to + var.len();

                    if equal_sign_exist(&code, vf, vt) {
                        let exind = track_until_nl(&code, vt) - 1;
                        let varval = get_expression(&code, vt, exind).unwrap().replace("\"", "");
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

                        stack.clear();
                        return;
                    } else {
                        println!("Expected Equal Operator Within Column {}-{}", vf, vt);
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
}
