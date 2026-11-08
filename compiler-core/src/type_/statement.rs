use crate::{
    analyzer::SemanticAnalyzer,
    ast::{
        TypedStmt,
        expression::Expr,
        fold_vec,
        identifier::Identifier,
        prelude::*,
        statement::{Stmt, StmtFolder},
    },
    error::{Error, Result, label, primary},
    lexer::Source,
    type_::{Typed, Untyped, prelude::*, unify},
};

impl<'a> StmtFolder for SemanticAnalyzer<'a> {
    type Type = Untyped;
    type Output = Result<TypedStmt>;

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, source: Source) -> Self::Output {
        let statements = fold_vec(statements, |stmt| self.fold_stmt(stmt))?;
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
        let type_ = if let Some(declared) = declared {
            unify(initializer.type_().clone(), declared.clone()).map_err(|d| {
                Error::new(
                    d.with_label(primary(source))
                        .with_label(label(format!("declared type was {}", declared), &name))
                        .with_label(label(
                            format!("initializer has type {}", initializer.type_()),
                            &initializer,
                        )),
                )
            })?;
            declared
        } else {
            initializer.type_().clone()
        };
        self.environment.declare_var(&name, type_.clone())?;
        Ok(let_(name, Some(type_), initializer, source))
    }

    fn fold_return(&mut self, value: Option<Expr<Self::Type>>, source: Source) -> Self::Output {
        let value = if let Some(value) = value {
            Some(self.infer(value)?)
        } else {
            None
        };
        Ok(return_(value, source))
    }

    fn fold_break(&mut self, source: Source) -> Self::Output {
        Ok(break_(source))
    }

    fn fold_if(
        &mut self,
        condition: Expr<Self::Type>,
        if_branch: Vec<Stmt<Self::Type>>,
        else_branch: Option<Vec<Stmt<Self::Type>>>,
        source: Source,
    ) -> Self::Output {
        let condition = self.infer_and_unify(condition, bool())?;
        let if_branch = fold_vec(if_branch, |stmt| self.fold_stmt(stmt))?;
        let else_branch = if let Some(else_branch) = else_branch {
            Some(fold_vec(else_branch, |stmt| self.fold_stmt(stmt))?)
        } else {
            None
        };
        Ok(if_(condition, if_branch, else_branch, source))
    }

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Vec<Stmt<Self::Type>>,
        source: Source,
    ) -> Self::Output {
        let condition = self.infer_and_unify(condition, bool())?;
        let body = fold_vec(body, |stmt| self.fold_stmt(stmt))?;
        Ok(while_(condition, body, source))
    }

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, source: Source) -> Self::Output {
        let expr = self.infer(expr)?;
        Ok(expr_stmt(expr, source))
    }
}
