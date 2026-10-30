use crate::{
    analyser::SemanticAnalyser,
    ast::{
        Identifier, Location, TypedDecl, TypedExpr, UntypedExpr,
        declaration::{DeclFolder, DeclFunc, DeclKind, DeclStruct},
        expression::{Expr, ExprFolder, ExprKind},
        origin::{BinaryOp, ExprOrigin, Literal, UnaryOp},
        prelude::*,
        statement::{Stmt, StmtKind},
    },
    error::{Error, Result, diagnostic, error},
    lexer::Source,
    type_::{Type, Typed, Untyped, collapse_links, environment::Environment, prelude::*, unify},
};
use codespan_reporting::diagnostic::Diagnostic;
use std::{boxed::Box, collections::HashMap, sync::Arc};

impl DeclFolder for SemanticAnalyser {
    type Type = Untyped;
    type Output = Result<TypedDecl>;

    fn fold_import(&mut self, path: Vec<Identifier>, source: Source) -> Self::Output {
        Ok(import(path, source))
    }

    fn fold_struct(&mut self, s: DeclStruct, source: Source) -> Self::Output {
        Ok(struct_(s, source))
    }

    fn fold_function(&mut self, f: DeclFunc<Self::Type>, source: Source) -> Self::Output {
        let (name, return_type, params, body, modifiers) =
            (f.name, f.return_type, f.params, f.body, f.modifiers);
        self.environment.begin_scope(HashMap::new());
        for (name, type_) in &params {
            self.environment.declare_var(name, type_.clone())?
        }
        let body = self.fold_stmts(body)?;
        self.environment.end_scope();
        Ok(func(
            DeclFunc::new(name, return_type, params, body, modifiers),
            source,
        ))
    }

    fn fold_const(
        &mut self,
        name: Identifier,
        type_: Typed,
        initializer: Expr<Self::Type>,
        source: Source,
    ) -> Self::Output {
        let initializer = self.infer_and_unify(initializer, type_.clone())?;
        Ok(const_(name, type_, initializer, source))
    }
}
