use crate::ir::{instruction::Instr, type_::Type, value::Value};

#[derive(Debug)]
pub enum Stmt {
    Assign(Value, Type, Instr),
    Volatile(Instr),
}
