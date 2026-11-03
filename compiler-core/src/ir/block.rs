use crate::ir::{statement::Stmt, instruction::Instr, type_::Type, value::Value};
use symbol_table::{static_symbol, GlobalSymbol};

#[derive(Debug)]
pub struct Block {
    pub name: GlobalSymbol,
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(name: GlobalSymbol) -> Self {
        Self { name, statements: Vec::new() }
    }

    pub fn assign_instr(&mut self, name: Value, type_: Type, instr: Instr) {
        self.statements.push(Stmt::Assign(name, type_, instr));
    }

    pub fn volatile_instr(&mut self, instr: Instr) {
        self.statements.push(Stmt::Volatile(instr))
    }
}

impl Default for Block {
    fn default() -> Self {
        Self { name: static_symbol!(""), statements: Vec::new() }
    }
}