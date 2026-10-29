use crate::{
    ast::{
        UntypedDecl,
        declaration::{DeclFunc, DeclStruct, DeclVisitor},
        expression::Expr,
        identifier::Identifier,
    },
    error::{FileDb, Result},
    lexer::Source,
    process_file,
    type_::{
        Type,
        environment::{Environment, Scope, TypeDefinition},
        prelude::Typed,
    },
    uid::UniqueIdGenerator,
};
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use symbol_table::GlobalSymbol;

pub struct Header {
    scope: Scope,
}

impl Header {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
        }
    }

    pub fn env<'ctx>(mut self, declarations: &[UntypedDecl]) -> Result<Environment<'ctx>> {
        for declaration in declarations {
            self.visit_decl(declaration)?;
        }
        Ok(Environment::new(UniqueIdGenerator::new(), self.scope))
    }
}

impl<'a> DeclVisitor<'a> for Header {
    type Type = ();
    type Output = Result<()>;

    fn visit_import(&mut self, path: &[Identifier], _: Source) -> Self::Output {
        let filename = {
            let mut filename = PathBuf::new();
            for name in path {
                filename.push(name.name());
            }
            filename.set_extension("tau");
            filename
        };
        let content = fs::read_to_string(&filename).expect("Expected to open file");
        let mut files = FileDb::new();
        let display = filename.display();
        let file_id = files.add(&display, &content);
        match process_file(file_id, &content) {
            Ok((_, env)) => {
                // TODO: insert module
            }
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
    }

    fn visit_struct(&mut self, structure: &'a DeclStruct, source: Source) -> Self::Output {
        let struct_name = GlobalSymbol::from(structure.name.name());

        let mut fields = Vec::new();
        for (field_name, field_type) in &structure.fields {
            fields.push((field_name.name().to_string(), field_type.clone()));
        }
        self.scope.insert(
            struct_name,
            TypeDefinition::new(
                Arc::new(Type::Named {
                    name: GlobalSymbol::from(structure.name.name()),
                    arguments: vec![],
                }),
                source,
            ),
        );
        Ok(())
    }

    fn visit_function(&mut self, func: &'a DeclFunc<Self::Type>, source: Source) -> Self::Output {
        let mut arguments = Vec::new();
        for (_, type_) in &func.params {
            arguments.push(type_.clone());
        }
        self.scope.insert(
            GlobalSymbol::from(func.name.name()),
            TypeDefinition::new(
                Arc::new(Type::Fn {
                    arguments,
                    return_: func.return_type.clone(),
                }),
                source,
            ),
        );
        Ok(())
    }

    fn visit_const(
        &mut self,
        name: &'a Identifier,
        type_: &'a Typed,
        _: &Expr<Self::Type>,
        source: Source,
    ) -> Self::Output {
        self.scope.insert(
            GlobalSymbol::from(name.name()),
            TypeDefinition::new(type_.clone(), source),
        );
        Ok(())
    }
}
