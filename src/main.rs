#![allow(clippy::needless_return)]
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

use crate::{
    ast::declaration::Decl,
    cli::{ArgsBuilder, HELP_MESSAGE},
    compiler::{
        Compiler,
        cpp::{CppCodeGenerator, CppHeaderGenerator},
        set_output,
    },
    error::{FileDb, FileId, Result, error},
    header::Header,
    lexer::Lexer,
    parser::Parser,
    resolution::Resolution,
    typing::{TypeNames, TypeTree},
};
use std::{env, fs, path::PathBuf};

mod ast;
mod cli;
mod compiler;
mod error;
mod header;
mod lexer;
mod parser;
mod resolution;
mod typing;

fn main() -> Result<()> {
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

fn compile_file(filename: &str, args: &cli::Args) -> Result<()> {
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
                    compiler.compile(CppHeaderGenerator, &header_output)?;
                    let code_output = set_output(filename.as_ref(), args.output(), "cpp");
                    compiler.compile(CppCodeGenerator::new(), &code_output)?;
                }
                cli::Target::Cranelift => unimplemented!(),
            },
            Err(error) => {
                for diagnostic in error.diagnostics() {
                    let writer = StandardStream::stderr(ColorChoice::Always);
                    let config = codespan_reporting::term::Config::default();

                    let _ =
                        term::emit_to_io_write(&mut writer.lock(), &config, &files, &diagnostic);
                }
            }
        }
        Ok(())
    } else {
        return error(Diagnostic::error().with_message(format!(
            "tau: cannot access {}: No such file",
            filename.display()
        )));
    }
}

fn process_file(file_id: FileId, content: &str) -> Result<(Vec<Decl>, TypeTree, TypeNames)> {
    let lexer = Lexer::new(content.chars(), file_id);
    let parser = Parser::new(lexer.scan()?);
    let ast = parser.parse()?;
    let header = Header::new().headers(&ast)?;
    let (types, fields) = header.analysed();
    let fields = Resolution::new(&types, fields).resolve(&ast)?;
    Ok((ast, types, fields))
}

#[cfg(test)]
mod tests {
    use super::compile_file;
    use crate::{cli::ArgsBuilder, error, lexer::Lexer, parser::Parser};
    use std::{path::PathBuf, rc::Rc};

    #[test]
    fn pipeline_arrays() -> Result<()> {
        let args = ArgsBuilder::new().build();
        compile_file(&Rc::new(PathBuf::from("examples/arrays.tau")), &args)
    }

    #[test]
    fn pipeline_heap() -> Result<()> {
        let args = ArgsBuilder::new().build();
        compile_file(&Rc::new(PathBuf::from("examples/heap.tau")), &args)
    }

    #[test]
    fn pipeline_vec2() -> Result<()> {
        let args = ArgsBuilder::new().build();
        compile_file(&Rc::new(PathBuf::from("examples/vec2.tau")), &args)
    }

    #[test]
    fn infinite_scoping() -> Result<()> {
        let content = "{ { { { { { {} } } } } } }";
        let lexer = Lexer::new(content.chars(), Rc::new(PathBuf::new()));
        let mut parser = Parser::new(lexer.scan().unwrap());
        parser.stmt().map(|_| {})
    }
}
