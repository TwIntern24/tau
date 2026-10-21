#![allow(clippy::needless_return)]
use tau_core::{
    error::FileDb,
    generator::{
        Compiler,
        set_output,
    },
    process_file,
};
use tau_generator_c::{
    CppCodeGenerator, CppHeaderGenerator,
};
use crate::cli::{ArgsBuilder, HELP_MESSAGE};
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};
use std::{env, fs, path::PathBuf, process::exit};

mod cli;

type DynResult = std::result::Result<(), Box<dyn std::fmt::Debug>>;

pub fn main() {
    let args = ArgsBuilder::new()
        .parse(env::args().collect())
        .unwrap()
        .build();
    if args.input().is_empty() {
        println!("{}", HELP_MESSAGE)
    } else {
        // guaranteed to not be empty
        for filename in args.input() {
            if let Err(message) = dbg!(compile_file(filename, &args)) {
                println!("{:?}", dbg!(message));
                exit(2);
            }
        }
    }
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
