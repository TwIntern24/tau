use std::fmt::{self, Display, Formatter};
use symbol_table::GlobalSymbol;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Temporary(GlobalSymbol),
    Global(GlobalSymbol),
    Const(u64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Temporary(name) => write!(f, "%{}", name),
            Self::Global(name) => write!(f, "${}", name),
            Self::Const(value) => write!(f, "{}", value),
        }
    }
}
