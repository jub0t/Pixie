use std::{any::Any, collections::HashMap};

pub type AnyValue = Box<dyn Any>;
pub type Functions = HashMap<String, Function>;
pub type Variables = HashMap<String, Variable>;

#[derive(PartialEq, Debug, Clone)]
pub enum Keywords {
    NONE = 0,
    CONST = 1,
    LET = 2,
    FUNCTION = 3,
}

#[derive(PartialEq, Debug)]
pub enum Methods {
    NONE = 0,
    PRINT = 1,
    ECHO = 2,
}

#[derive(Debug)]
pub struct Variable {
    pub vtype: Type,
    pub mutable: bool,
    pub value: AnyValue,
}

#[derive(Debug, Clone)]
pub enum FuntionType {
    CUSTOM = 1,
}

#[derive(Debug)]
pub struct Action {
    pub ptype: Type,
    pub value: AnyValue,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub index: i8,
    pub name: String,
    pub ptype: Type,
    pub value: String,
}

#[derive(Debug)]
pub struct Argument {
    pub ptype: Type,
    pub value: AnyValue,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ftype: FuntionType,
    pub name: String,
    pub params: Vec<Param>,
    pub inner_contents: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
}
