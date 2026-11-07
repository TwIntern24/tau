use crate::{
    lexer::Source,
    type_::{
        Type,
        prelude::{Typed, Untyped},
    },
};
use std::sync::Arc;

pub mod declaration;
pub mod expression;
pub mod identifier;
pub mod lookup;
pub mod origin;
pub mod prelude;
pub mod statement;

pub use declaration::Decl;
pub use expression::Expr;
pub use identifier::Identifier;
pub use statement::Stmt;

pub trait Unfold {
    type Unfolded;

    fn unfold(self) -> Self::Unfolded;
}

pub trait Location {
    fn source(&self) -> Source;

    fn merge(&self, location: &impl Location) -> Source {
        self.source().merge(location.source())
    }
}

impl<L> From<&L> for Source
where
    L: Location,
{
    fn from(value: &L) -> Source {
        value.source()
    }
}

pub fn fold_vec<I, O, E, F>(fold: Vec<I>, mut func: F) -> Result<Vec<O>, E>
where
    F: FnMut(I) -> Result<O, E>,
{
    let mut folded = Vec::with_capacity(fold.len());
    for e in fold {
        folded.push(func(e)?);
    }
    Ok(folded)
}

pub type UntypedExpr = Expr<Untyped>;
pub type UntypedStmt = Stmt<Untyped>;
pub type UntypedDecl = Decl<Untyped>;
pub type UntypedAst = Vec<UntypedDecl>;

pub type TypedExpr = Expr<Arc<Type>>;
pub type TypedStmt = Stmt<Arc<Type>>;
pub type TypedDecl = Decl<Arc<Type>>;
pub type TypedAst = Vec<TypedDecl>;
