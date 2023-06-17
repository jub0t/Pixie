use std::any::Any;

#[derive(Debug)]
pub enum Keywords {
    LET,
    CONST,
    NONE,
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
    pub value: Box<dyn Any>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
}
