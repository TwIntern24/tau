use crate::ir::{block::Block, type_::Type};
use symbol_table::GlobalSymbol;

pub struct Linkage {
    pub exported: bool,
}

pub struct Function {
    pub linkage: Linkage,
    pub name: GlobalSymbol,
    pub arguments: Vec<(GlobalSymbol, Type)>,
    pub return_type: Option<Type>,
    pub blocks: Vec<Block>,
}
