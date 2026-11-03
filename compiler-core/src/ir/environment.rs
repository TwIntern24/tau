use crate::{ir::{value::Value}, uid::UniqueIdGenerator};
use std::collections::HashMap;
use symbol_table::GlobalSymbol;

pub type Scope = HashMap<GlobalSymbol, Value>;

#[derive(Debug)]
pub struct Environment {
    ids: UniqueIdGenerator,

    /// Values defined in the current function together with the name of the block.
    scopes: Vec<(GlobalSymbol, Scope)>,

    /// End labels for each block. They are the jmp destination for the break statement.
    loop_labels: Vec<GlobalSymbol>
}

impl Environment {
    pub fn new(ids: UniqueIdGenerator) -> Self {
        Self {
            ids,
            scopes: Vec::new(),
            loop_labels: Vec::new()
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

    pub fn substitude(&mut self, old: GlobalSymbol, new: Value) {
        self.scope_mut().insert(old, new);
    }

    pub fn resolve(&self, name: &GlobalSymbol) -> Value {
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

    pub fn new_tmp_var(&mut self) -> Value {
        Value::Temporary(format!("tmp${}", self.ids.next()).into())
    }

    pub fn new_data_name(&mut self) -> GlobalSymbol {
        format!("data${}", self.ids.next()).into()
    }

    pub fn new_block_name(&mut self) -> GlobalSymbol {
        format!("block${}", self.ids.next()).into()
    }
}
