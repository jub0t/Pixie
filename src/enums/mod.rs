use std::any::Any;

pub type AnyValue = Box<dyn Any>;

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
enum FuntionType {
    STANDARD,
    CUSTOM,
}

#[derive(Debug)]
pub struct Action {
    ptype: Type,
    value: AnyValue,
}

#[derive(Debug)]
pub struct Param {
    ptype: Type,
    value: AnyValue,
}

#[derive(Debug)]
pub struct Function {
    ftype: FuntionType,
    name: String,
    params: Vec<Param>,
    action: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
}
