use crate::type_::{Type, TypeVar};
use std::{cell::RefCell, sync::Arc};
use symbol_table::{GlobalSymbol, static_symbol};

pub type Untyped = ();
pub type Typed = Arc<Type>;

pub fn fn_(arguments: Vec<Arc<Type>>, return_: Arc<Type>) -> Arc<Type> {
    Arc::new(Type::Fn { return_, arguments })
}

pub fn named(name: GlobalSymbol, arguments: Vec<Arc<Type>>) -> Arc<Type> {
    Arc::new(Type::Named {
        name: name,
        arguments,
    })
}

fn native(name: GlobalSymbol) -> Typed {
    named(name, vec![])
}

pub fn bool() -> Typed {
    native(static_symbol!("bool"))
}

pub fn array(inner: Typed, size: usize) -> Typed {
    Arc::new(Type::Array { type_: inner, size })
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
