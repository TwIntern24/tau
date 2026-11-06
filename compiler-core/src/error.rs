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

pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// All results throw our error type.
pub type Result<T> = result::Result<T, Error>;

/// Caches the content of a single file.
#[derive(Clone, Debug, Default)]
pub struct FileCache(Rc<String>);

impl FileCache {
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
pub type FileId = usize; // <FileDb as Files>::FileId

#[derive(PartialEq, Debug, Default)]
pub struct Error(Vec<Diagnostic<FileId>>);

impl Error {
    pub fn new(diagnostic: Diagnostic<FileId>) -> Self {
        Self(vec![diagnostic])
    }

    pub fn push(&mut self, diagnostic: Diagnostic<FileId>) {
        self.0.push(diagnostic);
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn add_label(&mut self, label: Label<FileId>) {
        let labels = &mut self.0.last_mut().unwrap().labels;
        labels.push(label)
    }

    pub fn diagnostics(&self) -> &Vec<Diagnostic<FileId>> {
        &self.0
    }
}

impl From<Vec<Diagnostic<FileId>>> for Error {
    fn from(value: Vec<Diagnostic<FileId>>) -> Self {
        Self(value)
    }
}

pub fn diagnostic(message: impl Display, source: impl Into<Source>) -> Diagnostic<FileId> {
    let source = source.into();
    Diagnostic::error()
        .with_message(message)
        .with_label(Label::primary(source.file_id(), source.range()))
}

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
