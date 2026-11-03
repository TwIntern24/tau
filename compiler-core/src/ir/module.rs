use crate::ir::{data::DataDef, function::Function, type_::Type};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    pub types: Vec<Type>,
    pub data: Vec<DataDef>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            types: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for data in &self.data {
            write!(f, "{}\n", data)?;
        }

        for function in &self.functions {
            write!(f, "{}\n", function)?;
        }
        Ok(())
    }
}
