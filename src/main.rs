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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename as an argument.");
        return;
    }

    let file_name = args[1].clone();
    let bytes = fs::read(file_name);

    match bytes {
        Ok(bytes) => {
            let mut variables: HashMap<String, Variable> = HashMap::new();
            let mut functions: HashMap<String, Function> = HashMap::new();

            let code = String::from_utf8(bytes).unwrap();
            parse_raw_code(code.as_str(), &mut functions, &mut variables, vec![])
        }
        Err(err) => {
            println!("Error reading the file: {}", err);
        }
    }
}
