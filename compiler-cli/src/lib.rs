#![allow(clippy::needless_return)]
use crate::code_gen::CodeGen;
use crate::options::{Options, options};
use std::{fs, process::exit, time::Instant};
use tau_core::{
    ast::Unfold,
    context::Context,
    error::{DynResult, FileCache},
    generator::{Compiler, output},
    process_file,
    translator::Translator,
};
#[cfg(feature = "c")]
use tau_generator_c::{CCodeGenerator, CHeaderGenerator};
use tau_generator_qbe::QbeEmitter;

pub mod code_gen;
pub mod options;

pub fn main() {
    let args = options().run();
    let now = Instant::now();
    let mut context = Context::new();
    if let Err(message) = compile_file(&mut context, &args) {
        println!("{}", message);
        exit(2);
    }
    let elapsed = now.elapsed().as_millis() as f64 / 1000.0;
    if let Some(severity) = context.severity() {
        println!("{:?} after {}s", severity, elapsed);
    } else {
        println!("Finished in {}s", elapsed);
    }
}

pub fn compile_file(context: &mut Context, args: &Options) -> DynResult<()> {
    let filename = args.input();
    let content = fs::read_to_string(&filename)?;
    let cache = FileCache::new(content);
    let display = filename.display();
    let file_id = context.store(display.to_string(), cache.clone());
    match process_file(context, args.module_name(), file_id, cache.as_ref()) {
        Ok((ast, _env)) => match args.code_gen() {
            #[cfg(feature = "c")]
            CodeGen::C => {
                let folder = &args.out_dir()?;
                let compiler = Compiler::new(&ast);
                let header_output = output(folder, args.output(), "h");
                compiler.compile(CHeaderGenerator, &header_output)?;
                let code_output = output(folder, args.output(), "c");
                compiler.compile(CCodeGenerator::new(), &code_output)
            }
            CodeGen::Cranelift => unimplemented!(),
            CodeGen::Qbe => {
                let output = output(&args.out_dir()?, args.output(), "ssa");
                let mut emitter = QbeEmitter::new();
                Translator::new(&mut emitter).fold_ast(ast);
                Ok(fs::write(output, &format!("{}", emitter.unfold()))?)
            }
        },
        Err(error) => context.handle(&error),
    }
}
