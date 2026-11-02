use crate::ir::{linkage::Linkage, type_::Type};
use symbol_table::GlobalSymbol;

#[derive(Debug)]
pub enum DataItem {
    Symbol(String, Option<u64>),
    Str(String),
    Const(u64),
    Zero(u64),
}

#[derive(Debug)]
pub struct DataDef {
    pub linkage: Linkage,
    pub name: GlobalSymbol,
    pub align: Option<u64>,
    pub items: Vec<(Type, DataItem)>,
}
