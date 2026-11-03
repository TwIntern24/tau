use std::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    sync::Arc,
};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug)]
pub enum TypeDef {
    Regular {
        ident: GlobalSymbol,
        align: Option<u64>,
        items: Vec<(Type, u64)>,
    },
    Union {
        ident: GlobalSymbol,
        align: Option<u64>,
        variations: Vec<Vec<(Type, u64)>>,
    },
    Opaque {
        ident: GlobalSymbol,
        align: u64,
        size: u64,
    },
}

impl TypeDef {
    pub fn identifier(&self) -> GlobalSymbol {
        match self {
            Self::Regular { ident, .. }
            | Self::Union { ident, .. }
            | Self::Opaque { ident, .. } => *ident,
        }
    }

    pub fn sizeof(&self) -> u64 {
        match self {
            Self::Regular { items, .. } => {
                let sum: u64 = items
                    .iter()
                    .map(|(type_, count)| Type::sizeof(type_) * count)
                    .sum::<u64>();
                sum
            }
            Self::Union {
                align, variations, ..
            } => variations
                .iter()
                .map(|variation| {
                    variation
                        .iter()
                        .map(|(type_, count)| Type::sizeof(type_) * count)
                        .sum::<u64>()
                })
                .max()
                .or(*align)
                .unwrap_or_default(),
            Self::Opaque { size, .. } => *size,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Bool,
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
    Pointer(Box<Type>),
    Aggregate(Arc<TypeDef>),
}

impl Type {
    pub fn sizeof(&self) -> u64 {
        match self {
            Self::Bool | Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 | Self::Pointer(_) => 8,
            Self::Aggregate(type_def) => TypeDef::sizeof(type_def),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let type_ = match self {
            Self::Bool => static_symbol!("bool"),
            Self::U8 => static_symbol!("u8"),
            Self::U16 => static_symbol!("u16"),
            Self::U32 => static_symbol!("u32"),
            Self::U64 => static_symbol!("u64"),
            Self::I8 => static_symbol!("i8"),
            Self::I16 => static_symbol!("i16"),
            Self::I32 => static_symbol!("i32"),
            Self::I64 => static_symbol!("i64"),
            Self::F32 => static_symbol!("f32"),
            Self::F64 => static_symbol!("f64"),
            Self::Pointer(type_) => return write!(f, "*{}", type_),
            Self::Aggregate(type_def) => type_def.identifier(),
        };
        write!(f, "{}", type_)
    }
}
