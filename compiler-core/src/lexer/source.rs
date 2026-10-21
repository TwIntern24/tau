use crate::{
    ast::identifier::Identifier,
    error::{FileId, Result, error},
    lexer::token::Token,
};
use codespan_reporting::diagnostic::Diagnostic;
use std::ops::Range;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Source {
    file_id: FileId,
    range: Range<usize>,
}

impl Source {
    pub fn new(file_id: FileId, range: Range<usize>) -> Source {
        Source { file_id, range }
    }

    pub fn union(left: &Source, right: &Source) -> Source {
        Source {
            file_id: left.file_id,
            range: left.range.start..right.range.end,
        }
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl<'a> From<&'a Token> for &'a Source {
    fn from(value: &'a Token) -> &'a Source {
        value.source()
    }
}

impl TryFrom<&Vec<Identifier>> for Source {
    type Error = crate::error::Error;

    fn try_from(value: &Vec<Identifier>) -> Result<Source> {
        if value.is_empty() {
            return error(Diagnostic::error().with_message("expected that type path is not empty"));
        }
        Ok(Source::union(
            value
                .first()
                .expect("expect first identifier exists")
                .source(),
            value
                .last()
                .expect("expect last identifier exists")
                .source(),
        ))
    }
}
