use symbol_table::GlobalSymbol;
use std::{sync::Arc, cell::RefCell};
use crate::type_::{Type, TypeVar};

pub type Untyped = ();
pub type Typed = Arc<Type>;

pub fn fn_(arguments: Vec<Arc<Type>>, return_: Arc<Type>) -> Arc<Type> {
    Arc::new(Type::Fn { return_, arguments })
}

pub fn named(
    name: GlobalSymbol,
    arguments: Vec<Arc<Type>>,
) -> Arc<Type> {
    Arc::new(Type::Named {
        name: name,
        arguments,
    })
}

pub fn generic_var(id: u64) -> Arc<Type> {
    Arc::new(Type::Var {
        type_: Arc::new(RefCell::new(TypeVar::Generic { id })),
    })
}

pub fn unbound_var(id: u64) -> Arc<Type> {
    Arc::new(Type::Var {
        type_: Arc::new(RefCell::new(TypeVar::Unbound { id })),
    })
}

pub fn link(type_: Arc<Type>) -> Arc<Type> {
    Arc::new(Type::Var {
        type_: Arc::new(RefCell::new(TypeVar::Link { type_ })),
    })
}