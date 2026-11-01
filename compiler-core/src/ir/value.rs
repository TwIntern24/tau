use symbol_table::GlobalSymbol;

pub enum Value {
    Temporary(GlobalSymbol),
    Global(GlobalSymbol),
    Const(u64),
}
