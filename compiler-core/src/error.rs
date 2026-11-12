//! Custom error type for reporting compiler errors.
//!
//! An error is a list of diagnostics. Each diagnostic indicates a problem with
//! the code. The Tau compiler uses codespan reporting for better error
//! reporting.

use crate::lexer::Source;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
    result,
};

/// Result with an dynamic boxed error type.
pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// All results throw our error type.
pub type Result<T> = result::Result<T, Error>;

/// Caches the content of a single file.
#[derive(Clone, Debug, Default)]
pub struct FileCache(Rc<String>);

impl FileCache {
    /// Creates a new file cache from the given file content.
    pub fn new(content: String) -> Self {
        Self(Rc::new(content))
    }
}

impl AsRef<str> for FileCache {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

/// The file database that we currently use. This may change in the future.
pub type FileDb = SimpleFiles<String, FileCache>;
/// The file id type that the file db uses.
pub type FileId = usize;

#[derive(PartialEq, Debug, Default)]
pub struct Error(Vec<Diagnostic<FileId>>);

impl Error {
    /// Creates a new error from the given diagnostic.
    pub fn new(diagnostic: Diagnostic<FileId>) -> Self {
        Self(vec![diagnostic])
    }

    /// Pushes the diagnostic to the back of this error.
    pub fn push(&mut self, diagnostic: Diagnostic<FileId>) {
        self.0.push(diagnostic);
    }

    /// Extend this error by adding all diagnostics from the other error to this
    /// error.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    /// Adds the given label to the last diagnostic. This method will panic if
    /// the error does not contain any diagnostics.
    pub fn add_label(&mut self, label: Label<FileId>) {
        let labels = &mut self.0.last_mut().expect("diagnostics not empty").labels;
        labels.push(label)
    }

    /// Returns all diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic<FileId>] {
        &self.0
    }
}

impl From<Vec<Diagnostic<FileId>>> for Error {
    fn from(value: Vec<Diagnostic<FileId>>) -> Self {
        Self(value)
    }
}

/// Creates a new error diagnostic from the given message and source.
pub fn diagnostic(message: impl Display, source: impl Into<Source>) -> Diagnostic<FileId> {
    let source = source.into();
    Diagnostic::error()
        .with_message(message)
        .with_label(primary(source))
}

/// Creates a new primary label from the given source.
pub fn primary(source: impl Into<Source>) -> Label<FileId> {
    let source = source.into();
    Label::primary(source.file_id(), source.range())
}

pub fn label(message: impl Display, source: impl Into<Source>) -> Label<FileId> {
    let source = source.into();
    Label::secondary(source.file_id(), source.range()).with_message(message)
}

pub fn error<T>(diagnostic: Diagnostic<FileId>) -> Result<T> {
    Err(Error(vec![diagnostic]))
}
