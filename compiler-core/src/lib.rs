#![allow(unused)]

use crate::{
    analyser::SemanticAnalyser,
    ast::TypedDecl,
    error::{FileId, Result},
    header::Header,
    lexer::Lexer,
    parser::Parser,
    type_::environment::Environment,
};

pub mod analyser;
pub mod ast;
pub mod error;
pub mod generator;
pub mod header;
pub mod lexer;
pub mod parser;
pub mod type_;
pub mod uid;

pub fn process_file<'ctx>(
    file_id: FileId,
    content: &str,
) -> Result<(Vec<TypedDecl>, Environment<'ctx>)> {
    let lexer = Lexer::new(&content, file_id);
    let ast = Parser::new(lexer).parse()?;
    let env = { Header::new().env(&ast)? };
    let mut analyser = SemanticAnalyser::new(env);
    let ast = analyser.fold_ast(ast)?;
    let env = analyser.env();
    Ok((ast, env))
}
