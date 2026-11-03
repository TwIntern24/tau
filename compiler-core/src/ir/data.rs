use crate::ir::{linkage::Linkage, type_::Type};
use std::fmt::{self, Display, Formatter};
use symbol_table::GlobalSymbol;

#[derive(Debug)]
pub enum DataItem {
    Symbol(String, Option<u64>),
    Str(String),
    Const(u64),
    Zero(u64),
}

impl Display for DataItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Symbol(name, _) => write!(f, "{}", name),
            Self::Str(string) => write!(f, r#""{}""#, string),
            Self::Const(value) => write!(f, "{}", value),
            Self::Zero(len) => write!(f, "[z {}]", len),
        }
    }
}

#[derive(Debug)]
pub struct DataDef {
    pub linkage: Linkage,
    pub name: GlobalSymbol,
    pub align: Option<u64>,
    pub items: Vec<(Type, DataItem)>,
}

impl Display for DataDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let items = self
            .items
            .iter()
            .map(|(type_, item)| format!("{} {}", type_, item))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "data ${} = {{ {} }}", self.name, items)
    }
}
