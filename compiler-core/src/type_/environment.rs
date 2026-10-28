use symbol_table::GlobalSymbol;
use std::collections::HashMap;
use crate::{ast::Location, lexer::Source, type_::{Typed, prelude::*}, uid::UniqueIdGenerator};

#[derive(Debug)]
pub struct TypeDefinition {
    source: Source,
    type_: Typed
}

impl Location for TypeDefinition {
    fn source(&self) -> &Source {
        &self.source
    }
}

pub type Scope = HashMap<GlobalSymbol, TypeDefinition>;

#[derive(Debug)]
pub struct ModuleInterface {
    pub name: GlobalSymbol,
    pub values: Scope,
}

#[derive(Debug)]
pub struct Environment<'a> {
    ids: UniqueIdGenerator,
    
    /// Modules that have been imported by the current module, along with the
    /// location of the import statement where they were imported.
    modules: HashMap<GlobalSymbol, &'a ModuleInterface>,

    /// Values defined in the current function
    scopes: Vec<Scope>,
}

impl<'a> Environment<'a> {
    pub fn begin_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop().expect("scopes not empty");
    }

    pub fn scope(&self) -> &Scope {
        self.scopes.last().expect("scope exists")
    }

    pub fn scoped<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        self.begin_scope(HashMap::new());
        let result = f(self);
        self.end_scope();
        result
    }

    pub fn new_unbound_var(&mut self) -> Typed {
        unbound_var(self.ids.next())
    }


}