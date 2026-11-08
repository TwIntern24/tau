use crate::{
    ast::{Location, Typed, expression::Expr, identifier::Identifier},
    lexer::Source,
    type_::Type,
};
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub struct Stmt<T> {
    kind: StmtKind<T>,
    source: Source,
}

impl<T> Stmt<T> {
    pub fn new(kind: StmtKind<T>, source: Source) -> Self {
        Stmt { kind, source }
    }

    pub fn kind(&self) -> &StmtKind<T> {
        &self.kind
    }
}

impl<T> Location for Stmt<T> {
    fn source(&self) -> Source {
        self.source
    }
}

#[derive(PartialEq, Debug)]
pub enum StmtKind<T> {
    Block {
        statements: Vec<Stmt<T>>,
    },
    Let {
        name: Identifier,
        declared: Option<Arc<Type>>,
        initializer: Expr<T>,
    },
    Return {
        value: Option<Expr<T>>,
    },
    Break,
    If {
        condition: Expr<T>,
        if_branch: Vec<Stmt<T>>,
        else_branch: Option<Vec<Stmt<T>>>,
    },
    While {
        condition: Expr<T>,
        body: Vec<Stmt<T>>,
    },
    ExprStmt(Expr<T>),
}

pub trait StmtVisitor<'a> {
    type Type;
    type Output;

    fn visit_stmt(&mut self, stmt: &'a Stmt<Self::Type>) -> Self::Output {
        match stmt.kind() {
            StmtKind::Block { statements } => self.visit_block(statements),
            StmtKind::Let {
                name,
                declared,
                initializer,
            } => self.visit_let(name, declared, initializer),
            StmtKind::Return { value } => self.visit_return(value),
            StmtKind::Break => self.visit_break(),
            StmtKind::If {
                condition,
                if_branch,
                else_branch,
            } => self.visit_if(condition, if_branch, else_branch),
            StmtKind::While { condition, body } => self.visit_while(condition, body),
            StmtKind::ExprStmt(expr) => self.visit_expr_stmt(expr),
        }
    }

    fn visit_block(&mut self, statements: &'a [Stmt<Self::Type>]) -> Self::Output;

    fn visit_let(
        &mut self,
        name: &'a Identifier,
        declared: &'a Option<Typed>,
        initializer: &'a Expr<Self::Type>,
    ) -> Self::Output;

    fn visit_return(&mut self, value: &'a Option<Expr<Self::Type>>) -> Self::Output;

    fn visit_break(&mut self) -> Self::Output;

    fn visit_if(
        &mut self,
        condition: &'a Expr<Self::Type>,
        if_branch: &'a [Stmt<Self::Type>],
        else_branch: &'a Option<Vec<Stmt<Self::Type>>>,
    ) -> Self::Output;

    fn visit_while(
        &mut self,
        condition: &'a Expr<Self::Type>,
        body: &'a [Stmt<Self::Type>],
    ) -> Self::Output;

    fn visit_expr_stmt(&mut self, expr: &'a Expr<Self::Type>) -> Self::Output;
}

pub trait StmtFolder {
    type Type;
    type Output;

    fn fold_stmt(&mut self, stmt: Stmt<Self::Type>) -> Self::Output {
        let (kind, source) = (stmt.kind, stmt.source);
        match kind {
            StmtKind::Block { statements } => self.fold_block(statements, source),
            StmtKind::Let {
                name,
                declared,
                initializer,
            } => self.fold_let(name, declared, initializer, source),
            StmtKind::Return { value } => self.fold_return(value, source),
            StmtKind::Break => self.fold_break(source),
            StmtKind::If {
                condition,
                if_branch,
                else_branch,
            } => self.fold_if(condition, if_branch, else_branch, source),
            StmtKind::While { condition, body } => self.fold_while(condition, body, source),
            StmtKind::ExprStmt(expr) => self.fold_expr_stmt(expr, source),
        }
    }

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, source: Source) -> Self::Output;

    fn fold_let(
        &mut self,
        name: Identifier,
        declared: Option<Typed>,
        initializer: Expr<Self::Type>,
        source: Source,
    ) -> Self::Output;

    fn fold_return(&mut self, value: Option<Expr<Self::Type>>, source: Source) -> Self::Output;

    fn fold_break(&mut self, source: Source) -> Self::Output;

    fn fold_if(
        &mut self,
        condition: Expr<Self::Type>,
        if_branch: Vec<Stmt<Self::Type>>,
        else_branch: Option<Vec<Stmt<Self::Type>>>,
        source: Source,
    ) -> Self::Output;

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Vec<Stmt<Self::Type>>,
        source: Source,
    ) -> Self::Output;

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, source: Source) -> Self::Output;
}
