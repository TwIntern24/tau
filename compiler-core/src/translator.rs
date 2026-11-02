#![allow(warnings)]
use crate::{
    ast::{
        declaration::{DeclFolder, FuncDecl, StructDecl},
        expression::{Expr, ExprFolder},
        identifier::Identifier,
        origin::{BinaryOp, ExprOrigin, Literal, UnaryOp},
        statement::{Stmt, StmtFolder},
    },
    ir::{
        block::Block,
        data::{DataDef, DataItem},
        function::Function,
        instruction::{Cmp, Instr},
        linkage::Linkage,
        module::Module,
        statement as ir_stmt,
        type_::{Type, TypeDef},
        value::Value,
    },
    lexer::Source,
    type_::{self, TypeVar, prelude::*},
    uid::UniqueIdGenerator,
};
use std::{ops::Deref, sync::Arc};
use symbol_table::GlobalSymbol;

/// Translates an AST to IR.
pub struct Translator {
    module: Module,
    blocks: Box<Vec<Block>>,
    stmts: Box<Vec<ir_stmt::Stmt>>,
    uid: UniqueIdGenerator,
}

impl Translator {
    fn next_data(&mut self) -> GlobalSymbol {
        format!("data${}", self.uid.next()).into()
    }

    fn next_tmp(&mut self) -> Value {
        Value::Temporary(format!("tmp${}", self.uid.next()).into())
    }

    fn save_temp_instr(&mut self, type_: Type, instr: Instr) -> Value {
        let value = self.next_tmp();
        let stmt = ir_stmt::Stmt::Assign(value, type_, instr);
        self.stmts.push(stmt);
        value
    }

    fn save_volatile_instr(&mut self, instr: Instr) {
        self.stmts.push(ir_stmt::Stmt::Volatile(instr));
    }
}

impl From<&Typed> for Type {
    fn from(value: &Typed) -> Type {
        match value.as_ref() {
            type_::Type::Fn { .. } => panic!(),
            type_::Type::Named { name, .. } => match name.as_str() {
                "f32" => Type::F32,
                "f64" => Type::F64,
                _ => todo!("handle struct"),
            },
            type_::Type::Var { type_ } => match type_.borrow().deref() {
                TypeVar::Unbound { .. } | TypeVar::Generic { .. } => panic!(),
                TypeVar::Link { type_ } => type_.into(),
            },
            type_::Type::Ptr { type_, .. } | type_::Type::Array { type_, .. } => type_.into(),
        }
    }
}

impl From<Typed> for Type {
    fn from(value: Typed) -> Type {
        Type::from(&value)
    }
}

impl DeclFolder for Translator {
    type Type = Typed;
    type Output = ();

    fn fold_import(&mut self, _: Vec<Identifier>, _: Source) -> Self::Output {}

    fn fold_struct(&mut self, struct_: StructDecl, _: Source) -> Self::Output {
        let ident = struct_.name.symbol();
        let align = None;
        // TODO: fold types
        let items = struct_
            .fields
            .into_iter()
            .map(|(_, v)| ((&v).into(), 1))
            .collect();

        self.module
            .types
            .push(Type::Aggregate(Arc::new(TypeDef::Regular {
                ident,
                align,
                items,
            })));
    }

    fn fold_function(&mut self, func: FuncDecl<Self::Type>, _: Source) -> Self::Output {
        let linkage = Linkage { exported: true };
        let name = func.name.symbol();
        let arguments = func
            .params
            .into_iter()
            .map(|(k, v)| (k.symbol(), (&v).into()))
            .collect();
        let return_type = if func.return_type.is_void() {
            None
        } else {
            Some((&func.return_type).into())
        };
        // TODO: fold statements
        let blocks = vec![];

        self.module.functions.push(Function {
            linkage,
            name,
            arguments,
            return_type,
            blocks,
        });
    }

    fn fold_const(
        &mut self,
        _: Identifier,
        _: Typed,
        _: Expr<Self::Type>,
        _: Source,
    ) -> Self::Output {
    }
}

impl StmtFolder for Translator {
    type Type = Typed;
    type Output = ();

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, source: Source) -> Self::Output {
        todo!()
    }

    fn fold_let(
        &mut self,
        name: Identifier,
        declared: Option<Typed>,
        initializer: Expr<Self::Type>,
        source: Source,
    ) -> Self::Output {
        todo!()
    }

    fn fold_return(&mut self, value: Expr<Self::Type>, source: Source) -> Self::Output {
        todo!()
    }

    fn fold_break(&mut self, source: Source) -> Self::Output {
        todo!()
    }

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Box<Stmt<Self::Type>>,
        source: Source,
    ) -> Self::Output {
        todo!()
    }

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, source: Source) -> Self::Output {
        todo!()
    }
}

impl ExprFolder for Translator {
    type Type = Typed;
    type Output = Value;

    fn fold_unary(
        &mut self,
        operator: ExprOrigin<UnaryOp>,
        right: Expr<Self::Type>,
        type_: Self::Type,
    ) -> Self::Output {
        let type_ = Type::from(type_);
        let right = self.fold_expr(right);
        match operator.kind() {
            UnaryOp::Neg => self.save_temp_instr(type_, Instr::Neg(right)),
            UnaryOp::Not => self.save_temp_instr(type_, Instr::Not(right)),
            UnaryOp::Ref => self.save_temp_instr(type_.clone(), Instr::Load(type_, right)),
            UnaryOp::Deref => {
                let left = self.next_tmp();
                let instr = Instr::Store(type_.clone(), left, right);
                self.save_temp_instr(type_, instr)
            }
        }
    }

    fn fold_binary(
        &mut self,
        left: Expr<Self::Type>,
        operator: ExprOrigin<BinaryOp>,
        right: Expr<Self::Type>,
        type_: Self::Type,
    ) -> Self::Output {
        let type_ = Type::from(type_);
        let left = self.fold_expr(left);
        let right = self.fold_expr(right);
        match operator.kind() {
            BinaryOp::Add => self.save_temp_instr(type_, Instr::Add(left, right)),
            BinaryOp::Sub => self.save_temp_instr(type_, Instr::Sub(left, right)),
            BinaryOp::Mul => self.save_temp_instr(type_, Instr::Mul(left, right)),
            BinaryOp::Div => self.save_temp_instr(type_, Instr::Div(left, right)),
            BinaryOp::Mod => self.save_temp_instr(type_, Instr::Mod(left, right)),
            BinaryOp::Set => {
                self.save_volatile_instr(Instr::Mov(left, right));
                left
            }
            BinaryOp::SetAdd => {
                self.save_volatile_instr(Instr::Add(left, right));
                left
            }
            BinaryOp::SetSub => {
                self.save_volatile_instr(Instr::Sub(left, right));
                left
            }
            BinaryOp::SetMul => {
                self.save_volatile_instr(Instr::Mul(left, right));
                left
            }
            BinaryOp::SetDiv => {
                self.save_volatile_instr(Instr::Div(left, right));
                left
            }

            BinaryOp::And => self.save_temp_instr(Type::Bool, Instr::And(left, right)),
            BinaryOp::Or => self.save_temp_instr(Type::Bool, Instr::Or(left, right)),

            BinaryOp::Eq => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Eq, left, right))
            }
            BinaryOp::Neq => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Ne, left, right))
            }
            BinaryOp::Low => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Slt, left, right))
            }
            BinaryOp::Leq => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Sle, left, right))
            }
            BinaryOp::Gre => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Sgt, left, right))
            }
            BinaryOp::Geq => {
                self.save_temp_instr(Type::Bool, Instr::Cmp(type_, Cmp::Sge, left, right))
            }
        }
    }

    fn fold_access_struct(
        &mut self,
        left: Expr<Self::Type>,
        right: Identifier,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_index_array(
        &mut self,
        object: Expr<Self::Type>,
        index: Expr<Self::Type>,
        source: Source,
        type_: Self::Type,
    ) -> Self::Output {
        todo!()
    }

    fn fold_call(
        &mut self,
        callee: Expr<Self::Type>,
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
        todo!()
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
        condition: Expr<Self::Type>,
        if_branch: Box<Stmt<Self::Type>>,
        else_branch: Option<Box<Stmt<Self::Type>>>,
        _: Source,
        type_: Self::Type,
    ) -> Self::Output {
        let condition = self.fold_expr(condition);
        let if_branch = self.fold_stmt(*if_branch);
        if let Some(else_branch) = else_branch {
            let else_branch = self.fold_stmt(*else_branch);
        }
        // We can not compile this, because if may return nothing!
        // To translate this, we need to change the ast first.
        todo!()
    }

    fn fold_literal(&mut self, value: ExprOrigin<Literal>, type_: Self::Type) -> Self::Output {
        match value.kind() {
            Literal::Int(value) => Value::Const(*value as u64),
            Literal::Float(value) => Value::Const((*value).to_bits()),
            Literal::Bool(value) => Value::Const(*value as u64),
            Literal::String(value) => {
                let linkage = Linkage { exported: false };
                let name = self.next_data();
                let align = None;
                let items = vec![
                    (Type::U8, DataItem::Str(value.into())),
                    // C ABI strings are null terminated.
                    (Type::U8, DataItem::Const(0)),
                ];

                self.module.data.push(DataDef {
                    linkage,
                    name,
                    align,
                    items,
                });

                Value::Global(name)
            }
        }
    }

    fn fold_variable(&mut self, name: Identifier, _: Self::Type) -> Self::Output {
        Value::Temporary(name.into())
    }
}
