use std::collections::HashMap;

use crate::{interpreter::Primitive, parse::Function};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Bool,
    String,
    I32,
    I64,
    F32,
    F64,
    Array(Box<Type>),
}

pub struct Compiler{
    pub variable_map: HashMap<String, (Primitive, Type)>,
    pub function_map: HashMap<String, Function>,
    pub variable_type_map: HashMap<String, Type>,
}
impl Compiler{
    pub fn new() -> Compiler {
        Compiler {
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            variable_type_map: HashMap::new(),
        }
    }
}