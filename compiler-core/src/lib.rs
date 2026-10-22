use crate::{
    ast::declaration::Decl,
    error::{FileId, Result},
    header::Header,
    lexer::Lexer,
    parser::Parser,
    resolution::Resolution,
    typing::{TypeNames, TypeTree},
};

pub mod ast;
pub mod error;
pub mod generator;
pub mod header;
pub mod lexer;
pub mod parser;
pub mod resolution;
pub mod type_;
pub mod typing;

pub fn process_file(file_id: FileId, content: &str) -> Result<(Vec<Decl>, TypeTree, TypeNames)> {
    let lexer = Lexer::new(&content, file_id);
    let parser = Parser::new(lexer);
    let ast = parser.parse()?;
    let header = Header::new().headers(&ast)?;
    let (types, fields) = header.analysed();
    let fields = Resolution::new(&types, fields).resolve(&ast)?;
    Ok((ast, types, fields))
}
