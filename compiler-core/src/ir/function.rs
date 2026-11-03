use crate::ir::{block::Block, linkage::Linkage, type_::Type};
use std::fmt::{self, Display, Formatter};
use symbol_table::{GlobalSymbol, static_symbol};

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
        Self {
            linkage: Linkage::public(),
            name: static_symbol!(""),
            arguments: Vec::new(),
            return_type: None,
            blocks: Vec::new(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let arguments = self
            .arguments
            .iter()
            .map(|(name, type_)| format!("%{}: {}", name, type_))
            .collect::<Vec<String>>()
            .join(", ");
        let return_type = if let Some(return_type) = &self.return_type {
            format!("-> {} ", return_type)
        } else {
            "".to_string()
        };
        let blocks = self
            .blocks
            .iter()
            .map(|block| format!("{}", block))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "fn ${}({}) {}{{\n{}\n}}",
            self.name, arguments, return_type, blocks
        )
    }
}
