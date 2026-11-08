//! Context object that owns and manages the core data of Tau.

use crate::{
    error::{DynResult, Error, FileCache, FileDb, FileId},
    type_::{environment::ModuleInterface, uid::UniqueIdGenerator},
};
use codespan_reporting::{
    diagnostic::Severity,
    term::{
        self, Config,
        termcolor::{ColorChoice, StandardStream},
    },
};
use std::{collections::HashMap, sync::Arc};
use symbol_table::GlobalSymbol;

/// The context owns and manages the core data of Tau. This data is persistent
/// between multiple steps. It contains everything from representations of
/// module interfaces to file caches for error reporting.
pub struct Context {
    files: FileDb,
    writer: StandardStream,
    config: Config,
    /// Id generator for unbound types.
    ids: UniqueIdGenerator,
    /// All modules that were read with this context.
    modules: HashMap<GlobalSymbol, Arc<ModuleInterface>>,
    /// Highest severity that were handled by this context.
    severity: Option<Severity>,
}

impl Context {
    /// Creates a new context object without modules or file caches.
    pub fn new() -> Self {
        Self {
            files: FileDb::new(),
            writer: StandardStream::stderr(ColorChoice::Always),
            config: Config::default(),
            ids: UniqueIdGenerator::new(),
            modules: HashMap::new(),
            severity: None,
        }
    }

    /// Handles the error. This will write all diagnostics with the context
    /// style and file db to the error stream.
    pub fn handle(&mut self, error: &Error) -> DynResult<()> {
        for diagnostic in error.diagnostics() {
            self.severity = Some(
                self.severity
                    .map_or_else(|| diagnostic.severity, |s| s.max(diagnostic.severity)),
            );
            term::emit_to_write_style(
                &mut self.writer.lock(),
                &self.config,
                &self.files,
                diagnostic,
            )?;
        }
        Ok(())
    }

    /// Stores the file caches under the given name and returns a unique file
    /// id.
    pub fn store(&mut self, name: String, source: FileCache) -> FileId {
        self.files.add(name, source)
    }

    /// Returns the UID generator. This is used to generate ids for unbound
    /// types.
    pub fn ids(&self) -> &UniqueIdGenerator {
        &self.ids
    }

    /// Caches the module interface. This will use the name of the module as the
    /// key.
    ///
    /// # Examples
    ///
    /// ```
    /// use tau_core::{context::Context, type_::environment::ModuleInterface};
    /// use std::sync::Arc;
    ///
    /// fn cache(context: &mut Context, module: Arc<ModuleInterface>) {
    ///     context.cache_module(module.clone());
    ///     assert!(context.find_module(&module.name().symbol()).is_some());
    /// }
    /// ```
    pub fn cache_module(&mut self, module: Arc<ModuleInterface>) {
        self.modules.insert(module.name().symbol(), module);
    }

    /// Finds the module by its name.
    ///
    /// # Examples
    ///
    /// ```
    /// use tau_core::{context::Context, type_::environment::ModuleInterface};
    /// use std::sync::Arc;
    ///
    /// fn find(context: &mut Context, module: Arc<ModuleInterface>) {
    ///     assert!(context.find_module(&module.name().symbol()).is_none());
    ///     context.cache_module(module.clone());
    ///     assert!(context.find_module(&module.name().symbol()).is_some());
    /// }
    /// ```
    pub fn find_module(&self, name: &GlobalSymbol) -> Option<&Arc<ModuleInterface>> {
        self.modules.get(name)
    }

    /// If any error was handled, returns the highest severity reported.
    /// Otherwise returns none.
    pub fn severity(&self) -> Option<Severity> {
        self.severity
    }
}
