//! Performs basic semantic analysis and type checking

use crate::{
    ast::{
        TypedAst, TypedExpr, UntypedAst, UntypedExpr, declaration::DeclFolder,
        expression::ExprFolder, fold_vec,
    },
    error::{Error, Result, label, primary},
    type_::{Typed, environment::Environment, unify},
};

pub struct SemanticAnalyzer<'a> {
    pub(crate) environment: &'a mut Environment,
}

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new analyzer from the given environment. This environment is
    /// normally created by the header.
    pub fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    /// Fold an untyped ast to a typed ast.
    pub fn fold_ast(&mut self, ast: UntypedAst) -> Result<TypedAst> {
        fold_vec(ast, |decl| self.fold_decl(decl))
    }

    /// Infers the type to the given expression.
    pub fn infer(&mut self, expr: UntypedExpr) -> Result<TypedExpr> {
        self.fold_expr(expr)
    }

    /// Infers the type of the given expression and tries to unify it with the
    /// given type, recording any unification error that might take place.
    pub fn infer_and_unify(&mut self, expression: UntypedExpr, type_: Typed) -> Result<TypedExpr> {
        let expression = self.infer(expression)?;
        unify(type_, expression.type_().clone()).map_err(|d| {
            Error::new(d.with_label(primary(&expression)).with_label(label(
                format!("expression has type {}", expression.type_()),
                &expression,
            )))
        })?;
        Ok(expression)
    }

    pub(crate) fn new_unbound_var(&mut self) -> Typed {
        self.environment.new_unbound_var()
    }
}
