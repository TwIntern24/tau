use crate::ir::{instruction::Instr, type_::Type, value::Value};

pub enum Stmt {
    Assign(Value, Type, Instr),
    Volatile(Instr),
}
