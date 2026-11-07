use crate::code_gen::CodeGen;
use bpaf::*;
use std::{env, io, path::PathBuf};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Options {
    #[bpaf(
        argument("DIR"),
        help("Write output to compiler-chosen filename in DIR")
    )]
    pub out_dir: Option<PathBuf>,
    #[bpaf(
        short('o'),
        long("output"),
        argument("FILENAME"),
        help("Write output to FILENAME")
    )]
    pub output: Option<PathBuf>,
    #[bpaf(
        argument("NAME"),
        help("Specify the NAME of the crate being built"),
        fallback(static_symbol!(""))
    )]
    pub crate_name: GlobalSymbol,
    #[bpaf(argument("NAME"), help("Specify the NAME of the module being built"))]
    pub module_name: Option<GlobalSymbol>,
    #[bpaf(
        short('C'),
        long("codegen"),
        argument("GEN"),
        help("Set the code generator"),
        fallback(CodeGen::default())
    )]
    pub code_gen: CodeGen,
    #[bpaf(positional("PATH"), complete_shell(ShellComp::File { mask: Some("*.tau") }))]
    pub input: PathBuf,
}

impl Options {
    pub fn new(
        input: PathBuf,
        out_dir: Option<PathBuf>,
        output: Option<PathBuf>,
        crate_name: GlobalSymbol,
        module_name: Option<GlobalSymbol>,
        code_gen: CodeGen,
    ) -> Self {
        Self {
            input,
            out_dir,
            output,
            crate_name,
            module_name,
            code_gen,
        }
    }

    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    pub fn out_dir(&self) -> io::Result<PathBuf> {
        self.out_dir
            .as_ref()
            .map_or_else(|| env::current_dir(), |c| Ok(c.to_path_buf()))
    }

    pub fn output(&self) -> &PathBuf {
        self.output.as_ref().unwrap_or_else(|| self.input())
    }

    pub fn crate_name(&self) -> GlobalSymbol {
        self.crate_name
    }

    pub fn module_name(&self) -> GlobalSymbol {
        self.module_name.unwrap_or_else(|| {
            self.input()
                .file_prefix()
                .map(|os_str| format!("{}", os_str.display()))
                .map_or_else(|| static_symbol!(""), |string| GlobalSymbol::new(string))
        })
    }

    pub fn code_gen(&self) -> &CodeGen {
        &self.code_gen
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            input: Default::default(),
            out_dir: Default::default(),
            output: Default::default(),
            crate_name: static_symbol!(""),
            module_name: Default::default(),
            code_gen: Default::default(),
        }
    }
}
