use crate::LlvmCodeGen;
use tau_core::{
    ast::{
        Identifier,
        expression::{Expr, ExprFolder},
        fold_vec,
        statement::{Stmt, StmtFolder},
    },
    error::DynResult,
    lexer::Source,
    type_::Typed,
};

impl<'a, 'ctx> StmtFolder for LlvmCodeGen<'a, 'ctx> {
    type Type = Typed;
    type Output = DynResult<()>;

    fn fold_block(&mut self, statements: Vec<Stmt<Self::Type>>, _: Source) -> Self::Output {
        fold_vec(statements, |statement| self.fold_stmt(statement))?;
        Ok(())
    }

    fn fold_let(
        &mut self,
        name: Identifier,
        declared: Option<Typed>,
        initializer: Expr<Self::Type>,
        _: Source,
    ) -> Self::Output {
        let name = name.symbol();

        let initializer = self.fold_expr(initializer)?;
        let alloca =
            self.build_entry_block_alloca(name.as_str(), &declared.expect("type resolved"));
        self.builder.build_store(alloca, initializer)?;
        self.variables.insert(name, alloca);
        Ok(())
    }

    fn fold_return(&mut self, value: Option<Expr<Self::Type>>, _: Source) -> Self::Output {
        if let Some(value) = value {
            let value = self.fold_expr(value)?;
            self.builder.build_return(Some(&value))?;
        } else {
            self.builder.build_return(None)?;
        };
        Ok(())
    }

    fn fold_break(&mut self, _: Source) -> Self::Output {
        todo!()
    }

    fn fold_if(
        &mut self,
        condition: Expr<Self::Type>,
        if_branch: Vec<Stmt<Self::Type>>,
        else_branch: Option<Vec<Stmt<Self::Type>>>,
        _: Source,
    ) -> Self::Output {
        let parent = self.fn_value();
        let condition = self.fold_expr(condition)?;
        let condition = self.as_condition(condition);

        // build branch
        let if_block = self.context.append_basic_block(parent, "if");
        let else_block = self.context.append_basic_block(parent, "else");
        let end_block = self.context.append_basic_block(parent, "end");

        self.builder
            .build_conditional_branch(condition, if_block, else_block)?;

        // if block
        self.builder.position_at_end(if_block);
        fold_vec(if_branch, |statement| self.fold_stmt(statement))?;
        self.builder.build_unconditional_branch(end_block)?;
        self.builder.get_insert_block().unwrap();

        // else block
        self.builder.position_at_end(else_block);
        if let Some(else_branch) = else_branch {
            fold_vec(else_branch, |statement| self.fold_stmt(statement))?;
        }
        self.builder.build_unconditional_branch(end_block)?;
        self.builder.get_insert_block().unwrap();

        // merge block
        self.builder.position_at_end(end_block);
        Ok(())
    }

    fn fold_while(
        &mut self,
        condition: Expr<Self::Type>,
        body: Vec<Stmt<Self::Type>>,
        _: Source,
    ) -> Self::Output {
        let parent = self.fn_value();

        let check_block = self.context.append_basic_block(parent, "check");
        let loop_block = self.context.append_basic_block(parent, "loop");
        let end_block = self.context.append_basic_block(parent, "end");

        // check if the condition holds
        self.builder.build_unconditional_branch(check_block)?;
        self.builder.position_at_end(check_block);
        let condition = self.fold_expr(condition)?;
        let condition = self.as_condition(condition);
        self.builder
            .build_conditional_branch(condition, loop_block, end_block)?;

        // execute the loop body
        self.builder.position_at_end(loop_block);
        fold_vec(body, |statement| self.fold_stmt(statement))?;
        self.builder.build_unconditional_branch(check_block)?;

        self.builder.position_at_end(end_block);

        Ok(())
    }

    fn fold_expr_stmt(&mut self, expr: Expr<Self::Type>, _: Source) -> Self::Output {
        self.fold_expr(expr)?;
        Ok(())
    }
}
