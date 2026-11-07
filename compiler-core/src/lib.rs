use crate::{
    analyser::SemanticAnalyser,
    ast::{TypedDecl, identifier::Identifier},
    context::Context,
    error::{FileId, Result},
    header::Header,
    lexer::{Lexer, Source},
    parser::Parser,
    type_::environment::Environment,
};
use symbol_table::GlobalSymbol;

pub mod analyser;
pub mod ast;
pub mod context;
pub mod error;
pub mod generator;
pub mod header;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod translator;
pub mod type_;
pub mod uid;

pub fn process_file<'ctx>(
    context: &'ctx mut Context,
    name: GlobalSymbol,
    file_id: FileId,
    content: &str,
) -> Result<(Vec<TypedDecl>, Environment)> {
    let lexer = Lexer::new(&content, file_id);
    let ast = Parser::new(lexer).parse()?;
    let env = Header::new(context, Identifier::new(name, Source::new(file_id, 0..0))).env(&ast)?;
    let mut analyzer = SemanticAnalyser::new(env);
    let ast = analyzer.fold_ast(ast)?;
    let env = analyzer.env();
    Ok((ast, env))
}
