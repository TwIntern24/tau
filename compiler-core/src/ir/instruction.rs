use symbol_table::GlobalSymbol;

use crate::ir::{type_::Type, value::Value};

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
    Blit(Value, Value, u64),
    DbgFile(GlobalSymbol),
    DbgLoc(u64, Option<u64>),
    Udiv(Value, Value),
    Urem(Value, Value),
    Sar(Value, Value),
    Shr(Value, Value),
    Shl(Value, Value),
    Cast(Value),
    Extsw(Value),
    Extuw(Value),
    Extsh(Value),
    Extuh(Value),
    Extsb(Value),
    Extub(Value),
    Exts(Value),
    Truncd(Value),
    Stosi(Value),
    Stoui(Value),
    Dtosi(Value),
    Dtoui(Value),
    Swtof(Value),
    Uwtof(Value),
    Sltof(Value),
    Ultof(Value),
    Vastart(Value),
    Vaarg(Type, Value),
    Phi(Vec<(GlobalSymbol, Value)>),
    Hlt,
}
