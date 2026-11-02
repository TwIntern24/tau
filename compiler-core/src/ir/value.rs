use symbol_table::GlobalSymbol;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Temporary(GlobalSymbol),
    Global(GlobalSymbol),
    Const(u64),
}
