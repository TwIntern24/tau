use crate::{
    ir::{aggregate::Aggregate, data::DataItem, instruction::Cmp, linkage::Linkage, value::Value},
    type_::Typed,
};
use symbol_table::GlobalSymbol;

pub trait Emitter {
    type Instr;

    fn begin_func(
        &mut self,
        linkage: Linkage,
        name: GlobalSymbol,
        arguments: Vec<(GlobalSymbol, Typed)>,
        return_type: Option<Typed>,
    );
    fn end_func(&mut self);

    fn begin_block(&mut self, label: GlobalSymbol);
    fn end_block(&mut self);

    fn layout_hint(&mut self, type_: &Typed) -> (u64, u64);

    fn emit_type_def(&mut self, aggregate: Aggregate);

    fn emit_data_def(
        &mut self,
        linkage: Linkage,
        name: GlobalSymbol,
        items: Vec<(Typed, DataItem)>,
    );

    fn emit_stmt_assign(&mut self, name: GlobalSymbol, type_: &Typed, init: Self::Instr);
    fn emit_stmt_volatile(&mut self, instr: Self::Instr);

    fn emit_stmt_jmp(&mut self, label: GlobalSymbol) {
        let jmp = self.emit_instr_jmp(label);
        self.emit_stmt_volatile(jmp);
    }
    fn emit_stmt_jnz(
        &mut self,
        condition: Value,
        if_label: GlobalSymbol,
        else_label: GlobalSymbol,
    ) {
        let jnz = self.emit_instr_jnz(condition, if_label, else_label);
        self.emit_stmt_volatile(jnz);
    }
    fn emit_stmt_ret(&mut self, value: Option<Value>) {
        let ret = self.emit_instr_ret(value);
        self.emit_stmt_volatile(ret);
    }
    fn emit_stmt_store(&mut self, dest: &Value, src: &Value) {
        let store = self.emit_instr_store(dest, src);
        self.emit_stmt_volatile(store);
    }

    fn emit_instr_add(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_sub(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_mul(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_div(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_mod(&mut self, left: Value, right: Value) -> Self::Instr;

    fn emit_instr_cmp(&mut self, cmp: Cmp, left: Value, right: Value) -> Self::Instr;

    fn emit_instr_and(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_or(&mut self, left: Value, right: Value) -> Self::Instr;
    fn emit_instr_xor(&mut self, left: Value, right: Value) -> Self::Instr;

    fn emit_instr_not(&mut self, right: Value) -> Self::Instr;
    fn emit_instr_neg(&mut self, right: Value) -> Self::Instr;
    fn emit_instr_copy(&mut self, value: Value) -> Self::Instr;

    fn emit_instr_jmp(&mut self, label: GlobalSymbol) -> Self::Instr;
    fn emit_instr_jnz(
        &mut self,
        condition: Value,
        if_label: GlobalSymbol,
        else_label: GlobalSymbol,
    ) -> Self::Instr;
    fn emit_instr_call(&mut self, func: GlobalSymbol, args: Vec<Value>) -> Self::Instr;
    fn emit_instr_ret(&mut self, value: Option<Value>) -> Self::Instr;

    fn emit_instr_alloca(&mut self, type_: Typed, align: Option<u64>) -> Self::Instr;
    fn emit_instr_store(&mut self, dest: &Value, src: &Value) -> Self::Instr;
    fn emit_instr_load(&mut self, type_: &Typed, src: &Value) -> Self::Instr;
    fn emit_instr_phi(&mut self, values: Vec<(GlobalSymbol, Value)>) -> Self::Instr;
}
