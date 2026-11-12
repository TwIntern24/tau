//! Abstract syntax tree that directly reflects what the user wrote.
//!
//! The AST is split in three different parts:
//!
//! - **Declarations** that declare a type or a function.
//! - **Statements** that are executed from top to bottom that change the state
//!   of the program.
//! - **Expressions** that express a value and are evaluated in operator
//!   precedence.

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

/// Tree node that can be unfolded into its parts.
pub trait Unfold {
    /// The parts of this value.
    type Unfolded;

    /// Consumes this value and moves out all parts.
    fn unfold(self) -> Self::Unfolded;
}

/// Value that has a location in the source code associated with it. The source
/// code location is tracked for showing better compiler errors.
pub trait Location {
    /// Returns the source code location.
    fn source(&self) -> Source;

    /// Merge this location with the other location. This function always
    /// expects that this location was defined before other and that both are
    /// defined in the same file.
    fn merge(&self, other: &impl Location) -> Source {
        self.source().merge(other.source())
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

/// Fold this vector by applying func on all elements. The supplied function
/// must throw a result. If an error in a function call occurs, it is directly
/// reported.
///
/// # Examples
///
/// ```
/// use tau_core::{analyzer::SemanticAnalyzer, ast::{TypedStmt, UntypedStmt, fold_vec, statement::StmtFolder}, error::Result};
///
/// fn fold_block(
///     analyzer: &mut SemanticAnalyzer,
///     block: Vec<UntypedStmt>
/// ) -> Result<Vec<TypedStmt>> {
///     fold_vec(block, |stmt| analyzer.fold_stmt(stmt))
/// }
/// ```
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

/// Expression without type information.
pub type UntypedExpr = Expr<Untyped>;
/// Statement without type information.
pub type UntypedStmt = Stmt<Untyped>;
/// Declaration without type information.
pub type UntypedDecl = Decl<Untyped>;
/// AST without type information.
pub type UntypedAst = Vec<UntypedDecl>;

/// Expression with type information.
pub type TypedExpr = Expr<Arc<Type>>;
/// Statement with type information.
pub type TypedStmt = Stmt<Arc<Type>>;
/// Declaration with type information.
pub type TypedDecl = Decl<Arc<Type>>;
/// AST with type information.
pub type TypedAst = Vec<TypedDecl>;
