use crate::{
    analyser::SemanticAnalyser,
    ast::{
        TypedStmt,
        expression::Expr,
        identifier::Identifier,
        prelude::*,
        statement::{Stmt, StmtFolder},
    },
    error::{Result, diagnostic, error},
    lexer::Source,
    type_::{Typed, Untyped, environment::Environment, prelude::*, unify},
};

impl<'a> StmtFolder for SemanticAnalyser<'a> {
    type Type = Untyped;
    type Output = Result<TypedStmt>;

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, source: Source) -> Self::Output {
        let statements = self.fold_stmts(statements)?;
        Ok(block(statements, source))
    }

    fn fold_let(
        &mut self,
        name: Identifier,
        declared: Option<Typed>,
        initializer: Expr<Self::Type>,
        source: Source,
    ) -> Self::Output {
        let initializer = self.infer(initializer)?;
        let declared = if let Some(declared) = declared {
            unify(initializer.type_().clone(), declared.clone())?;
            Some(declared)
        } else {
            Some(initializer.type_().clone())
        };
        Ok(let_(name, declared, initializer, source))
    }

    fn fold_return(&mut self, value: Expr<Self::Type>, source: Source) -> Self::Output {
        let value = self.infer(value)?;
        Ok(return_(value, source))
    }

    fn fold_break(&mut self, source: Source) -> Self::Output {
        Ok(break_(source))
    }

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Box<Stmt<Self::Type>>,
        source: Source,
    ) -> Self::Output {
        let condition = self.infer_and_unify(condition, bool())?;
        let body = self.fold_stmt(*body)?;
        Ok(while_(condition, body, source))
    }

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, source: Source) -> Self::Output {
        let expr = self.infer(expr)?;
        Ok(expr_stmt(expr, source))
    }
}
