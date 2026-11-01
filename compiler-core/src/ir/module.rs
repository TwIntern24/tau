use crate::ir::{data::DataDef, function::Function, type_::Type};

pub struct Module {
    pub functions: Vec<Function>,
    pub types: Vec<Type>,
    pub data: Vec<DataDef>,
}
