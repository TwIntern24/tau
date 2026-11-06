use crate::type_::Typed;
use symbol_table::GlobalSymbol;

#[derive(Clone, Debug)]
pub struct Value {
    kind: ValueKind,
    type_: Typed,
}

impl Value {
    pub fn new(kind: ValueKind, type_: Typed) -> Self {
        Self { kind, type_ }
    }

    pub fn kind(&self) -> &ValueKind {
        &self.kind
    }

    pub fn type_(&self) -> &Typed {
        &self.type_
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ValueKind {
    Temporary(GlobalSymbol),
    Global(GlobalSymbol),
    AutoLoad(GlobalSymbol),
    Const(u64),
}
