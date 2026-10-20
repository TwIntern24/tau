#![allow(clippy::needless_return)]
use crate::{
    ast::declaration::Decl,
    cli::{ArgsBuilder, HELP_MESSAGE},
    compiler::{
        Compiler,
        cpp::{CppCodeGenerator, CppHeaderGenerator},
        set_output,
    },
    error::{FileDb, FileId, Result},
    header::Header,
    lexer::Lexer,
    parser::Parser,
    resolution::Resolution,
    typing::{TypeNames, TypeTree},
};
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};
use std::{env, fs, path::PathBuf, process::exit};

mod ast;
mod cli;
mod compiler;
mod error;
mod header;
mod lexer;
mod parser;
mod resolution;
mod typing;

type DynResult = std::result::Result<(), Box<dyn std::fmt::Debug>>;

fn main() -> DynResult {
    let args = ArgsBuilder::new()
        .parse(env::args().collect())
        .unwrap()
        .build();
    if args.input().is_empty() {
        println!("{}", HELP_MESSAGE)
    } else {
        // guaranteed to not be empty
        for filename in args.input() {
            compile_file(filename, &args)?;
        }
    }
    Ok(())
}

fn compile_file(filename: &str, args: &cli::Args) -> DynResult {
    let filename = PathBuf::from(filename);
    if let Ok(content) = fs::read_to_string(&filename) {
        let mut files = FileDb::new();
        let display = filename.display();
        let file_id = files.add(&display, &content);
        match process_file(file_id, &content) {
            Ok((ast, _, _)) => match args.target() {
                cli::Target::Cpp => {
                    let compiler = Compiler::new(&ast);
                    let header_output = set_output(filename.as_ref(), args.output(), "hpp");
                    compiler
                        .compile(CppHeaderGenerator, &header_output)
                        .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?;
                    let code_output = set_output(filename.as_ref(), args.output(), "cpp");
                    compiler
                        .compile(CppCodeGenerator::new(), &code_output)
                        .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)
                }
                cli::Target::Cranelift => unimplemented!(),
            },
            Err(error) => {
                for diagnostic in error.diagnostics() {
                    let writer = StandardStream::stderr(ColorChoice::Always);
                    let config = codespan_reporting::term::Config::default();

                    term::emit_to_io_write(&mut writer.lock(), &config, &files, &diagnostic)
                        .map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?;
                }
                Ok(())
            }
        }
    } else {
        println!("tau: cannot access {}: No such file", filename.display());
        exit(1);
    }
}

fn process_file(file_id: FileId, content: &str) -> Result<(Vec<Decl>, TypeTree, TypeNames)> {
    let lexer = Lexer::new(&content, file_id);
    let parser = Parser::new(lexer);
    let ast = parser.parse()?;
    let header = Header::new().headers(&ast)?;
    let (types, fields) = header.analysed();
    let fields = Resolution::new(&types, fields).resolve(&ast)?;
    Ok((ast, types, fields))
}

#[cfg(test)]
mod tests {
    use super::{DynResult, compile_file};
    use crate::{cli::ArgsBuilder, error::Result, lexer::Lexer, parser::Parser};

    #[test]
    fn pipeline_arrays() -> DynResult {
        let args = ArgsBuilder::new().build();
        compile_file("examples/arrays.tau", &args)
    }

    #[test]
    fn pipeline_heap() -> DynResult {
        let args = ArgsBuilder::new().build();
        compile_file("examples/heap.tau", &args)
    }

    #[test]
    fn pipeline_vec2() -> DynResult {
        let args = ArgsBuilder::new().build();
        compile_file("examples/vec2.tau", &args)
    }

    #[test]
    fn infinite_scoping() -> Result<()> {
        let content = "{ { { { { { {} } } } } } }";
        let lexer = Lexer::new(content, 0);
        let mut parser = Parser::new(lexer);
        parser.stmt().map(|_| {})
    }
}
