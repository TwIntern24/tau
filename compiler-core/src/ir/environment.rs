use crate::ir::{type_::Type, value::Value};
use std::collections::HashMap;
use symbol_table::{GlobalSymbol, static_symbol};

pub type Scope = HashMap<GlobalSymbol, Value>;

#[derive(Debug)]
pub struct Environment {
    types: HashMap<GlobalSymbol, Type>,

    /// Values defined in the current function together with the name of the block.
    scopes: Vec<(GlobalSymbol, Scope)>,

    /// End labels for each block. They are the jmp destination for the break statement.
    loop_labels: Vec<GlobalSymbol>,

    data_counter: usize,
    instr_counter: usize,
    block_counter: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            loop_labels: Vec::new(),
            types: {
                let mut types = HashMap::new();
                // TODO: replace void with empty tuple.
                types.insert(static_symbol!("void"), Type::U8);
                types.insert(static_symbol!("bool"), Type::Bool);
                types.insert(static_symbol!("u8"), Type::U8);
                types.insert(static_symbol!("u16"), Type::U16);
                types.insert(static_symbol!("u32"), Type::U32);
                types.insert(static_symbol!("u64"), Type::U64);
                types.insert(static_symbol!("i8"), Type::I8);
                types.insert(static_symbol!("i16"), Type::I16);
                types.insert(static_symbol!("i32"), Type::I32);
                types.insert(static_symbol!("i64"), Type::I64);
                types.insert(static_symbol!("f32"), Type::F32);
                types.insert(static_symbol!("f64"), Type::F64);
                types.insert(static_symbol!("str"), Type::U8);
                types
            },
            data_counter: 0,
            instr_counter: 0,
            block_counter: 0,
        }
    }

    pub fn begin_scope(&mut self, exit_label: GlobalSymbol) {
        self.scopes.push((exit_label, Scope::new()));
    }

    pub fn end_scope(&mut self) {
        self.scopes
            .pop()
            .expect("same number of open scopes as closed scopes");
    }

    pub fn begin_loop(&mut self, exit_label: GlobalSymbol) {
        self.loop_labels.push(exit_label);
    }

    pub fn end_loop(&mut self) {
        self.loop_labels
            .pop()
            .expect("same number of open scopes as closed scopes");
    }

    fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scopes.last_mut().expect("scope exists").1
    }

    pub fn declare(&mut self, name: GlobalSymbol, type_: Type) {
        self.types.insert(name, type_);
    }

    pub fn substitude(&mut self, old: GlobalSymbol, new: Value) {
        self.scope_mut().insert(old, new);
    }

    pub fn resolve_type(&self, name: &GlobalSymbol) -> &Type {
        self.types
            .get(name)
            .expect(&format!("expected type {name} was declared"))
    }

    pub fn resolve_var(&self, name: &GlobalSymbol) -> Value {
        for (_, scope) in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return *value;
            }
        }

        Value::Global(*name)
    }

    pub fn block_exit_label(&self) -> GlobalSymbol {
        self.scopes.last().expect("block label exists").0
    }

    pub fn loop_exit_label(&self) -> &GlobalSymbol {
        self.loop_labels.last().expect("loop label exists")
    }

    pub fn reset_counters(&mut self) {
        self.instr_counter = 0;
        self.block_counter = 0;
    }

    pub fn new_tmp_var(&mut self) -> Value {
        Value::Temporary(self.new_instr_name())
    }

    pub fn new_instr_name(&mut self) -> GlobalSymbol {
        let name = format!("ti{}", self.instr_counter).into();
        self.instr_counter += 1;
        name
    }

    pub fn new_data_name(&mut self) -> GlobalSymbol {
        let name = format!("td{}", self.data_counter).into();
        self.data_counter += 1;
        name
    }

    pub fn new_block_name(&mut self) -> GlobalSymbol {
        let name = format!("tb{}", self.block_counter).into();
        self.block_counter += 1;
        name
    }
}
