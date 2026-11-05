use crate::{
    ast::{Location, Unfold, expression::Expr, identifier::Identifier},
    lexer::Source,
};
use std::boxed::Box;

pub type VarLookup = NameLookup<Lookup>;
pub type CallLookup<T> = NameLookup<AdlLookup<T>>;

/// Name lookups are largely influenced by C++
#[derive(PartialEq, Debug)]
pub enum Lookup {
    /// <https://en.cppreference.com/cpp/language/qualified_lookup>
    Qualified(Vec<Identifier>),
    /// <https://en.cppreference.com/cpp/language/unqualified_lookup>
    Unqualified,
}

#[derive(PartialEq, Debug)]
pub enum AdlLookup<T> {
    Lookup(Lookup),
    /// Argument dependent loopup
    /// <https://en.cppreference.com/cpp/language/adl>
    Adl(Box<Expr<T>>),
}

#[derive(PartialEq, Debug)]
pub struct NameLookup<L> {
    name: Identifier,
    kind: L,
}

impl<L> NameLookup<L> {
    pub fn new(name: Identifier, kind: L) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn kind(&self) -> &L {
        &self.kind
    }
}

impl<T> NameLookup<AdlLookup<T>> {
    pub fn qualified(name: Identifier, identifiers: Vec<Identifier>) -> Self {
        Self {
            name,
            kind: AdlLookup::Lookup(Lookup::Qualified(identifiers)),
        }
    }

    pub fn unqualified(name: Identifier) -> Self {
        Self {
            name,
            kind: AdlLookup::Lookup(Lookup::Unqualified),
        }
    }

    pub fn adl(name: Identifier, expr: Expr<T>) -> Self {
        Self {
            name,
            kind: AdlLookup::Adl(Box::new(expr)),
        }
    }
}

impl<L> Unfold for NameLookup<L> {
    type Unfolded = (Identifier, L);

    fn unfold(self) -> Self::Unfolded {
        (self.name, self.kind)
    }
}

impl<L> Location for NameLookup<L> {
    fn source(&self) -> Source {
        self.name.source()
    }
}
