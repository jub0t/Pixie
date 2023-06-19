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

            for (_, line) in to_parse.iter().enumerate() {
                let code = line.as_str();
                let chars = code.clone().split("").enumerate();
                let spaces = get_all_space_indexes(code);
                let mut stack = "".to_string();

                for (index, character) in chars.clone() {
                    stack = stack.to_string() + character;
                    let kw = keyword_to_enum(stack.clone());

                    parse_keyword(
                        kw,
                        code,
                        index,
                        line,
                        &mut functions,
                        &mut variables,
                        &mut stack,
                        &spaces,
                    );

                    parse_builtin_method(stack.clone(), code, &variables);
                }

                parse_custom_function(stack, &functions);
            }
        }
        Err(err) => {
            println!("Error reading the file: {}", err);
        }
    }
}
