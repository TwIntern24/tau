use crate::ir::{block::Block, linkage::Linkage, type_::Type};
use symbol_table::{static_symbol, GlobalSymbol};

#[derive(Debug)]
pub struct Function {
    pub linkage: Linkage,
    pub name: GlobalSymbol,
    pub arguments: Vec<(GlobalSymbol, Type)>,
    pub return_type: Option<Type>,
    pub blocks: Vec<Block>,
}

impl Default for Function {
    fn default() -> Self {
        Self { linkage: Linkage::public(), name: static_symbol!(""), arguments: Vec::new(), return_type: None, blocks: Vec::new() }
    }
}
