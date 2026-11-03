use crate::ir::{instruction::Instr, type_::Type, value::Value};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Stmt {
    Assign(Value, Type, Instr),
    Volatile(Instr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Assign(value, type_, instr) => write!(f, "{}: {} = {}", value, type_, instr),
            Self::Volatile(instr) => instr.fmt(f),
        }
    }
}
