use crate::type_::Typed;
use symbol_table::GlobalSymbol;

pub enum Aggregate {
    Struct {
        name: GlobalSymbol,
        items: Vec<Typed>,
    },
    Union {
        name: GlobalSymbol,
        variations: Vec<Vec<Typed>>,
    },
}
