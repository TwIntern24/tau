use crate::{
    error::{DynResult, Error, FileDb, FileId},
    type_::environment::ModuleInterface,
    uid::UniqueIdGenerator,
};
use codespan_reporting::term::{
    self, Config,
    termcolor::{ColorChoice, StandardStream},
};
use std::{collections::HashMap, path, sync::Arc};
use symbol_table::GlobalSymbol;

pub struct Context {
    files: FileDb,
    writer: StandardStream,
    config: Config,
    ids: UniqueIdGenerator,
    modules: HashMap<GlobalSymbol, Arc<ModuleInterface>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            files: FileDb::new(),
            writer: StandardStream::stderr(ColorChoice::Always),
            config: Config::default(),
            ids: UniqueIdGenerator::new(),
            modules: HashMap::new(),
        }
    }

    pub fn handle(&mut self, error: &Error) -> DynResult {
        for diagnostic in error.diagnostics() {
            term::emit_to_io_write(
                &mut self.writer.lock(),
                &self.config,
                &self.files,
                diagnostic,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?;
        }
        Ok(())
    }

    pub fn store(&mut self, name: String, source: String) -> FileId {
        self.files.add(name, source)
    }

    pub fn ids(&self) -> &UniqueIdGenerator {
        &self.ids
    }

    pub fn cache_module(&mut self, module: Arc<ModuleInterface>) {
        self.modules.insert(module.name(), module);
    }

    pub fn find_module(&self, name: &GlobalSymbol) -> Option<&Arc<ModuleInterface>> {
        self.modules.get(name)
    }
}
