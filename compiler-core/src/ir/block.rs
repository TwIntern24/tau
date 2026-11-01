use crate::ir::statement::Stmt;
use symbol_table::GlobalSymbol;

pub struct Block {
    pub name: GlobalSymbol,
    pub statements: Vec<Stmt>,
}
