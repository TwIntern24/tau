use crate::{
    ast::{
        Identifier, Location, TypedExpr, UntypedExpr,
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

#[derive(Debug)]
pub(crate) struct ExprTyper<'a, 'b> {
    environment: &'a mut Environment<'b>,
}

impl<'a, 'b> ExprTyper<'a, 'b> {
    pub fn new(environment: &'a mut Environment<'b>) -> Self {
        Self { environment }
    }

    pub fn new_unbound_var(&mut self) -> Arc<Type> {
        self.environment.new_unbound_var()
    }

    pub fn infer(&mut self, expr: UntypedExpr) -> Result<TypedExpr> {
        self.fold_expr(expr)
    }

    /// Infers the type of the given function and tries to unify it with the
    /// given type, recording any unification error that might take place.
    /// The typed expression is returned in any case.
    pub fn infer_and_unify(
        &mut self,
        expression: UntypedExpr,
        type_: Arc<Type>,
    ) -> Result<TypedExpr> {
        let expression = self.infer(expression)?;
        unify(type_, expression.type_().clone())?;
        Ok(expression)
    }
}

impl<'a, 'b> ExprFolder for ExprTyper<'a, 'b> {
    type Type = Untyped;
    type Output = Result<TypedExpr>;

    fn fold_unary(
        &mut self,
        operator: ExprOrigin<UnaryOp>,
        right: Box<Expr<Self::Type>>,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_binary(
        &mut self,
        left: Box<Expr<Self::Type>>,
        operator: ExprOrigin<BinaryOp>,
        right: Box<Expr<Self::Type>>,
        type_: Self::Type,
    ) -> Self::Output {
        Ok(match operator.kind() {
            // (number -> number) -> number
            BinaryOp::Add
            | BinaryOp::Sub
            | BinaryOp::Mul
            | BinaryOp::Div
            | BinaryOp::Mod
            | BinaryOp::Set
            | BinaryOp::SetAdd
            | BinaryOp::SetSub
            | BinaryOp::SetMul
            | BinaryOp::SetDiv => {
                let left = self.infer(*left)?;
                let right = self.infer(*right)?;
                if !left.type_().is_number() || !right.type_().is_number() {
                    return error(diagnostic(
                        "left and right side must be numbers",
                        left.merge(&right),
                    ));
                }
                unify(left.type_().clone(), right.type_().clone())?;
                let type_ = left.type_().clone();
                binary(left, operator, right, type_)
            }

            // (a -> a) -> bool
            BinaryOp::Eq | BinaryOp::Neq => {
                let left = self.infer(*left)?;
                let right = self.infer(*right)?;
                unify(left.type_().clone(), right.type_().clone())?;
                binary(left, operator, right, bool())
            }

            // (number -> number) -> bool
            BinaryOp::Low | BinaryOp::Leq | BinaryOp::Gre | BinaryOp::Geq | BinaryOp::SetDiv => {
                let left = self.infer(*left)?;
                let right = self.infer(*right)?;
                if !left.type_().is_number() || !right.type_().is_number() {
                    return error(diagnostic(
                        "left and right side must be numbers",
                        left.merge(&right),
                    ));
                }
                unify(left.type_().clone(), right.type_().clone())?;
                binary(left, operator, right, bool())
            }

            // (bool -> bool) -> bool
            BinaryOp::And | BinaryOp::Or => {
                let left = self.infer(*left)?;
                let right = self.infer(*right)?;
                unify(left.type_().clone(), bool())?;
                unify(right.type_().clone(), bool())?;
                binary(left, operator, right, bool())
            }
        })
    }

    fn fold_get(
        &mut self,
        left: Box<Expr<Self::Type>>,
        right: Identifier,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_index(
        &mut self,
        object: Box<Expr<Self::Type>>,
        index: Box<Expr<Self::Type>>,
        source: Source,
        _: Self::Type,
    ) -> Self::Output {
        let object = Box::new(self.infer(*object)?);
        let index = Box::new(self.infer(*index)?);
        if !index.type_().is_int() {
            return error(diagnostic("index is not an integer", source));
        }
        match collapse_links(object.type_().clone()).as_ref() {
            Type::Array { type_, .. } => Ok(TypedExpr::new(ExprKind::IndexArray {
                object,
                index,
                source,
                type_: type_.clone(),
            })),
            _ => error(diagnostic("not an array", source)),
        }
    }

    fn fold_call(
        &mut self,
        callee: Box<Expr<Self::Type>>,
        arguments: Vec<Expr<Self::Type>>,
        source: Source,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_create_array(
        &mut self,
        fields: Vec<Expr<Self::Type>>,
        source: Source,
        type_: Typed,
    ) -> Self::Output {
        if let Type::Array {
            type_: inner_type,
            size,
        } = type_.as_ref()
        {
            let fields = {
                let mut typed_fields = Vec::with_capacity(fields.len());
                for field in fields {
                    let field = self.infer_and_unify(field, inner_type.clone())?;
                    typed_fields.push(field)
                }
                typed_fields
            };
            Ok(TypedExpr::new(ExprKind::CreateArray {
                fields,
                source,
                type_,
            }))
        } else {
            error(diagnostic("not an array type", source))
        }
    }

    fn fold_create_struct(
        &mut self,
        fields: Vec<(Identifier, Expr<Self::Type>)>,
        source: Source,
        type_: Typed,
    ) -> Self::Output {
        todo!()
    }

    fn fold_if(
        &mut self,
        condition: Box<Expr<Self::Type>>,
        if_branch: Box<Stmt<Self::Type>>,
        else_branch: Option<Box<Stmt<Self::Type>>>,
        source: Source,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_literal(&mut self, value: ExprOrigin<Literal>, type_: Self::Type) -> Self::Output {
        todo!()
    }

    fn fold_variable(&mut self, name: Identifier, type_: Self::Type) -> Self::Output {
        todo!()
    }
}
