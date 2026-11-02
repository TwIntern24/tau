use crate::ir::{block::Block, linkage::Linkage, type_::Type};
use symbol_table::GlobalSymbol;

#[derive(Debug)]
pub struct Function {
    pub linkage: Linkage,
    pub name: GlobalSymbol,
    pub arguments: Vec<(GlobalSymbol, Type)>,
    pub return_type: Option<Type>,
    pub blocks: Vec<Block>,
}
