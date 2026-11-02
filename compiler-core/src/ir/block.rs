use crate::ir::statement::Stmt;
use symbol_table::GlobalSymbol;

#[derive(Debug)]
pub struct Block {
    pub name: GlobalSymbol,
    pub statements: Vec<Stmt>,
}
