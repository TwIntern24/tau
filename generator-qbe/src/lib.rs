use std::{fs::File, io::Write};
use tau_core::{ast::Decl, error::DynResult, generator::Generator, type_::prelude::*};

pub struct QbeGenerator;

impl<'a> Generator<'a> for QbeGenerator {
    fn generate(&mut self, _: &'a [Decl<Typed>], output: &std::path::Path) -> DynResult<()> {
        let mut file = File::create(output).expect("error reading file");

        write!(file, "")?;
        Ok(())
    }
}
