use crate::{
    ast::{
        declaration::{Decl, DeclVisitor, Function, Structure},
        expression::Expr,
        identifier::Identifier,
    },
    error::{FileDb, Result},
    process_file,
    typing::{self, TypeCell, TypeDef, TypeNames, TypeTree},
};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::{collections::HashMap, fs, path::PathBuf, rc::Rc};

pub struct Header {
    types: TypeTree,
    scope: TypeNames,
}

impl Header {
    pub fn new() -> Header {
        Header {
            types: TypeTree::new(),
            scope: HashMap::new(),
        }
    }

    pub fn headers(mut self, declarations: &[Decl]) -> Result<Header> {
        for declaration in declarations {
            self.visit_decl(declaration)?;
        }
        Ok(self)
    }

    pub fn analysed(self) -> (TypeTree, TypeNames) {
        (self.types, self.scope)
    }
}

impl<'a> DeclVisitor<'a> for Header {
    type Output = Result<()>;

    fn visit_import(&mut self, path: &[Identifier]) -> Self::Output {
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
            Ok((_, types, scope)) => {
                self.types.insert_tree(
                    path.last().expect("import path not empty").to_string(),
                    types,
                );
                self.scope.extend(scope);
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

    fn visit_struct(&mut self, structure: &'a Structure) -> Self::Output {
        let struct_name = structure.name.name().to_string();

        let mut fields = HashMap::new();
        for (field_name, field_type) in &structure.fields {
            fields.insert(field_name.name().to_string(), field_type.borrow().clone());
        }
        self.types.insert_type(
            struct_name,
            Rc::new(TypeDef::Struct(typing::Struct {
                name: vec![structure.name.clone()],
                fields,
            })),
        );
        Ok(())
    }

    fn visit_function(&mut self, func: &'a Function) -> Self::Output {
        let mut parameters = Vec::new();
        for (_, param_type) in &func.params {
            parameters.push(param_type.borrow().clone());
        }
        self.scope.insert(
            func.name.name().to_string(),
            Rc::new(TypeDef::Function(typing::Function {
                name: vec![
                    // TODO: add full module path
                    func.name.clone(),
                ],
                parameters,
                return_type: func.return_type.borrow().clone(),
            })),
        );
        Ok(())
    }

    fn visit_const(
        &mut self,
        name: &'a Identifier,
        var_type: &'a TypeCell,
        _: &Expr,
    ) -> Self::Output {
        self.scope
            .insert(name.name().to_string(), var_type.borrow().clone());
        Ok(())
    }
}
