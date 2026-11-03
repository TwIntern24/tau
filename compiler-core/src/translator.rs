#![allow(warnings)]
use crate::{
    ast::{
        TypedDecl,
        declaration::{DeclFolder, FuncDecl, StructDecl},
        expression::{Expr, ExprFolder},
        identifier::Identifier,
        origin::{BinaryOp, ExprOrigin, Literal, UnaryOp},
        statement::{Stmt, StmtFolder},
    },
    ir::{
        block::Block,
        data::{DataDef, DataItem},
        environment::{Environment, Scope},
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
use std::{mem, ops::Deref, sync::Arc};
use symbol_table::{GlobalSymbol, static_symbol};

/// Translates an AST to IR.
pub struct Translator {
    /// Current constructed module.
    module: Module,

    /// Current constructed function.
    function: Function,

    /// Current constructed block.
    block: Block,

    /// Environment for managing substitutions.
    environment: Environment,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            function: Function::default(),
            block: Block::default(),
            environment: Environment::new(),
        }
    }

    pub fn fold_ast(mut self, ast: Vec<TypedDecl>) -> Module {
        for decl in ast {
            self.fold_decl(decl);
        }
        self.module
    }

    fn fold_type(&self, type_: Typed) -> Type {
        match type_.as_ref() {
            type_::Type::Fn { .. } => panic!(),
            type_::Type::Named { name, .. } => match name.as_str() {
                _ => self.environment.resolve_type(name).clone(),
            },
            type_::Type::Var { type_ } => match type_.borrow().deref() {
                TypeVar::Unbound { .. } | TypeVar::Generic { .. } => panic!(),
                TypeVar::Link { type_ } => self.fold_type(type_.clone()),
            },
            type_::Type::Ptr { type_, .. } | type_::Type::Array { type_, .. } => {
                self.fold_type(type_.clone())
            }
        }
    }

    fn save_temp_instr(&mut self, type_: Type, instr: Instr) -> Value {
        let name = self.environment.new_tmp_var();
        self.block.assign_instr(name, type_, instr);
        name
    }

    fn save_volatile_instr(&mut self, instr: Instr) {
        self.block.volatile_instr(instr);
    }
}

impl DeclFolder for Translator {
    type Type = Typed;
    type Output = ();

    fn fold_import(&mut self, _: Vec<Identifier>, _: Source) -> Self::Output {}

    fn fold_struct(&mut self, struct_: StructDecl, _: Source) -> Self::Output {
        let ident = struct_.name.symbol();
        let align = None;
        let items = struct_
            .fields
            .into_iter()
            .map(|(_, v)| (self.fold_type(v), 1))
            .collect();

        let type_ = Type::Aggregate(Arc::new(TypeDef::Regular {
            ident,
            align,
            items,
        }));
        self.environment
            .declare(struct_.name.symbol(), type_.clone());
        self.module.types.push(type_);
    }

    fn fold_function(&mut self, func: FuncDecl<Self::Type>, _: Source) -> Self::Output {
        self.function.linkage = Linkage { exported: true };
        self.function.name = func.name.symbol();
        self.function.return_type = if func.return_type.is_void() {
            None
        } else {
            Some(self.fold_type(func.return_type))
        };

        let func_end = self.environment.new_block_name();
        self.environment.begin_scope(func_end);
        for (old, type_) in func.params {
            let type_ = self.fold_type(type_);
            let new = self.environment.new_instr_name();
            self.environment
                .substitude(old.symbol(), Value::Temporary(new));
            self.function.arguments.push((new, type_));
        }
        for stmt in func.body {
            self.fold_stmt(stmt);
        }
        self.environment.end_scope();
        self.environment.reset_counters();

        self.function.blocks.push(mem::take(&mut self.block));

        self.module.functions.push(mem::take(&mut self.function));
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

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, _: Source) -> Self::Output {
        /// This is the name of the next block. We use this as a jmp destination if we need
        /// to leave the block earlier.
        let end_label = self.environment.new_block_name();
        self.environment.begin_scope(end_label);
        for stmt in statements {
            self.fold_stmt(stmt);
        }
        self.environment.end_scope();
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(end_label)));
    }

    fn fold_let(
        &mut self,
        old: Identifier,
        declared: Option<Typed>,
        initializer: Expr<Self::Type>,
        _: Source,
    ) -> Self::Output {
        let new = self.fold_expr(initializer);
        self.environment.substitude(old.symbol(), new);
    }

    fn fold_return(&mut self, value: Option<Expr<Self::Type>>, _: Source) -> Self::Output {
        let value = value.map(|v| self.fold_expr(v));
        self.save_volatile_instr(Instr::Ret(value))
    }

    fn fold_break(&mut self, _: Source) -> Self::Output {
        let label = self.environment.loop_exit_label();
        self.save_volatile_instr(Instr::Jmp(*label));
    }

    fn fold_if(
        &mut self,
        condition: Expr<Self::Type>,
        if_branch: Vec<Stmt<Self::Type>>,
        else_branch: Option<Vec<Stmt<Self::Type>>>,
        source: Source,
    ) -> Self::Output {
        // Define jmp labels
        let if_block = self.environment.new_block_name();
        let else_block = self.environment.new_block_name();
        let end_block = if else_branch.is_some() {
            self.environment.new_block_name()
        } else {
            else_block
        };

        // Jump if condition != 0
        let condition = self.fold_expr(condition);
        self.save_volatile_instr(Instr::Jnz(
            condition,
            if_block,
            if else_branch.is_some() {
                else_block
            } else {
                end_block
            },
        ));

        // Save if block
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(if_block)));
        self.environment.begin_scope(if_block);
        for stmt in if_branch {
            self.fold_stmt(stmt);
        }
        self.save_volatile_instr(Instr::Jmp(end_block));
        self.environment.end_scope();

        // Save else block
        if let Some(else_branch) = else_branch {
            self.function
                .blocks
                .push(mem::replace(&mut self.block, Block::new(else_block)));
            self.environment.begin_scope(else_block);
            for stmt in else_branch {
                self.fold_stmt(stmt);
            }
            self.save_volatile_instr(Instr::Jmp(end_block));
            self.environment.end_scope();
        }

        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(end_block)));
    }

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Vec<Stmt<Self::Type>>,
        source: Source,
    ) -> Self::Output {
        // Define jmp labels
        let check_block = self.environment.new_block_name();
        let while_block = self.environment.new_block_name();
        let end_block = self.environment.new_block_name();

        // Jump if condition != 0
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(check_block)));
        let condition = self.fold_expr(condition);
        self.save_volatile_instr(Instr::Jnz(condition, while_block, end_block));

        // Save while block
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(while_block)));
        self.environment.begin_scope(while_block);
        self.environment.begin_loop(end_block);
        for stmt in body {
            self.fold_stmt(stmt);
        }
        // Go back to the condition
        self.save_volatile_instr(Instr::Jmp(check_block));
        self.environment.end_loop();
        self.environment.end_scope();
    }

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, source: Source) -> Self::Output {
        // TODO: We drop the return value here. We drop it EVERY SINGLE TIME.
        // That's generelly not an issue, but we should eliminate the value in the future.
        self.fold_expr(expr);
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
        let type_ = self.fold_type(type_);
        let right = self.fold_expr(right);
        match operator.kind() {
            UnaryOp::Neg => self.save_temp_instr(type_, Instr::Neg(right)),
            UnaryOp::Not => self.save_temp_instr(type_, Instr::Not(right)),
            UnaryOp::Ref => self.save_temp_instr(type_.clone(), Instr::Load(type_, right)),
            UnaryOp::Deref => {
                let left = self.environment.new_tmp_var();
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
        let type_ = self.fold_type(type_);
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
        let struct_type = self.fold_type(left.type_().clone());
        // TODO: calculate offset from type
        let offset = 0;
        let left = self.fold_expr(left);
        let offset = self.save_temp_instr(struct_type, Instr::Add(left, Value::Const(offset)));
        let type_ = self.fold_type(type_);
        self.save_temp_instr(type_.clone(), Instr::Load(type_, offset))
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
        let callee = self.fold_expr(callee);
        let arguments = arguments
            .into_iter()
            .map(|expr| (self.fold_type(expr.type_().clone()), self.fold_expr(expr)))
            .collect::<Vec<(Type, Value)>>();

        // TODO: refactor ast to get function name
        let function_name = static_symbol!("unimplemented");
        self.save_temp_instr(
            self.fold_type(type_),
            Instr::Call(function_name, arguments, None),
        )
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
        _: Source,
        type_: Typed,
    ) -> Self::Output {
        let type_ = self.fold_type(type_);
        let size = Type::sizeof(&type_);
        let alloc = self.save_temp_instr(type_, Instr::Alloc8(size));
        let mut offset = 0;
        for (_, expr) in fields {
            let type_ = self.fold_type(expr.type_().clone());
            let expr = self.fold_expr(expr);
            let size = Type::sizeof(&type_);
            let pos = self.save_temp_instr(type_.clone(), Instr::Add(alloc, Value::Const(offset)));
            self.save_volatile_instr(Instr::Store(type_, alloc, expr));
            offset += size;
        }
        alloc
    }

    fn fold_ternary(
        &mut self,
        condition: Expr<Self::Type>,
        if_branch: Expr<Self::Type>,
        else_branch: Expr<Self::Type>,
        _: Source,
        type_: Self::Type,
    ) -> Self::Output {
        let if_block = self.environment.new_block_name();
        let else_block = self.environment.new_block_name();
        let end_block = self.environment.new_block_name();

        let condition = self.fold_expr(condition);
        self.save_volatile_instr(Instr::Jnz(condition, if_block, else_block));

        // Save if block
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(if_block)));
        self.environment.begin_scope(if_block);
        let if_branch = self.fold_expr(if_branch);
        self.save_volatile_instr(Instr::Jmp(end_block));
        self.environment.end_scope();

        // Save else block
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(else_block)));
        self.environment.begin_scope(else_block);
        let else_branch = self.fold_expr(else_branch);
        self.save_volatile_instr(Instr::Jmp(end_block));
        self.environment.end_scope();

        // Phi instruction
        self.function
            .blocks
            .push(mem::replace(&mut self.block, Block::new(end_block)));
        let type_ = self.fold_type(type_);
        self.save_temp_instr(
            type_,
            Instr::Phi(vec![(if_block, if_branch), (else_block, else_branch)]),
        )
    }

    fn fold_literal(&mut self, value: ExprOrigin<Literal>, type_: Self::Type) -> Self::Output {
        match value.kind() {
            Literal::Int(value) => Value::Const(*value as u64),
            Literal::Float(value) => Value::Const((*value).to_bits()),
            Literal::Bool(value) => Value::Const(*value as u64),
            Literal::String(value) => {
                let linkage = Linkage { exported: false };
                let name = self.environment.new_data_name();
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
        self.environment.resolve_var(&name.symbol())
    }
}
