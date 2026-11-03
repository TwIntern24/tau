use crate::ir::{type_::Type, value::Value};
use std::fmt::{self, Display, Formatter};
use symbol_table::GlobalSymbol;

#[derive(Debug)]
pub enum Cmp {
    /// Signed less then (<)
    Slt,
    /// Signed less equal (<=)
    Sle,
    /// Signed greater then (>)
    Sgt,
    /// Signed greater equal (>=)
    Sge,
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Unsigned less then (<)
    Ult,
    /// Unsigned less equal (<=)
    Ule,
    /// Unsigned greater then (>)
    Ugt,
    /// Unsigned greater equal (>=)
    Uge,
}

impl Display for Cmp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Slt | Self::Ult => write!(f, "<"),
            Self::Sle | Self::Ule => write!(f, "<="),
            Self::Sgt | Self::Ugt => write!(f, ">"),
            Self::Sge | Self::Uge => write!(f, ">="),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    // (number -> number) -> number
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),

    // a <- a
    Mov(Value, Value),

    Cmp(Type, Cmp, Value, Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Neg(Value),
    Not(Value),
    Copy(Value),
    Ret(Option<Value>),
    Jnz(Value, GlobalSymbol, GlobalSymbol),
    Jmp(GlobalSymbol),
    Call(GlobalSymbol, Vec<(Type, Value)>, Option<u64>),
    Alloc4(u32),
    Alloc8(u64),
    Alloc16(u128),
    Store(Type, Value, Value),
    Load(Type, Value),
    Phi(Vec<(GlobalSymbol, Value)>),
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Add(left, right) => write!(f, "add {}, {}", left, right),
            Self::Sub(left, right) => write!(f, "add {}, {}", left, right),
            Self::Mul(left, right) => write!(f, "add {}, {}", left, right),
            Self::Div(left, right) => write!(f, "add {}, {}", left, right),
            Self::Mod(left, right) => write!(f, "add {}, {}", left, right),
            Self::Mov(dest, src) => write!(f, "mov {}, {}", dest, src),
            Self::Cmp(_, cmp, left, right) => write!(f, "cmp ({}) {}, {}", cmp, left, right),
            Self::And(left, right) => write!(f, "and {}, {}", left, right),
            Self::Or(left, right) => write!(f, "or {}, {}", left, right),
            Self::Xor(left, right) => write!(f, "xor {}, {}", left, right),
            Self::Neg(value) => write!(f, "neg {}", value),
            Self::Not(value) => write!(f, "not {}", value),
            Self::Copy(value) => write!(f, "copy {}", value),
            Self::Ret(value) => {
                let value = value.map_or_else(|| "".to_string(), |v| format!(" {}", v));
                write!(f, "ret{}", value)
            }
            Self::Jnz(value, then_label, otherwise_label) => {
                write!(f, "jnz {}, @{} ? @{}", value, then_label, otherwise_label)
            }
            Self::Jmp(label) => write!(f, "jmp @{}", label),
            Self::Call(name, args, _) => {
                let args = args
                    .iter()
                    .map(|(type_, value)| format!("{}: {}", value, type_))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "call ${}({})", name, args)
            }
            Self::Alloc4(size) => write!(f, "alloc4 {}", size),
            Self::Alloc8(size) => write!(f, "alloc8 {}", size),
            Self::Alloc16(size) => write!(f, "alloc16 {}", size),
            Self::Store(_, dest, src) => write!(f, "store {}, {}", dest, src),
            Self::Load(_, value) => write!(f, "load {}", value),
            Self::Phi(labels) => {
                let labels = labels
                    .iter()
                    .map(|(label, value)| format!("@{}^{}", label, value))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "phi {}", labels)
            }
        }
    }
}
