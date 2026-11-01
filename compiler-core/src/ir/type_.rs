use std::sync::Arc;

use symbol_table::GlobalSymbol;

pub enum TypeDef {
    Regular {
        ident: GlobalSymbol,
        align: Option<u64>,
        items: Vec<(Type, usize)>,
    },
    Union {
        ident: GlobalSymbol,
        align: Option<u64>,
        variations: Vec<Vec<(Type, usize)>>,
    },
    Opaque {
        ident: GlobalSymbol,
        align: u64,
        size: u64,
    },
}

pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Aggregate(Arc<TypeDef>),
}
