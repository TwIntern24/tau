use crate::ir::{instruction::Instr, statement::Stmt, type_::Type, value::Value};
use std::fmt::{self, Display, Formatter};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug)]
pub struct Block {
    pub name: GlobalSymbol,
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(name: GlobalSymbol) -> Self {
        Self {
            name,
            statements: Vec::new(),
        }
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
        Self {
            name: static_symbol!(""),
            statements: Vec::new(),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let name = if self.name.as_str().is_empty() {
            "".to_string()
        } else {
            format!("@{}\n", self.name)
        };
        let stmts = self
            .statements
            .iter()
            .map(|stmt| format!("  {}", stmt))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}{}", name, stmts)
    }
}
