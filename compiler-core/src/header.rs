use crate::{
    ast::{
        Unfold, UntypedDecl,
        declaration::{DeclVisitor, FuncDecl, StructDecl},
        expression::Expr,
        identifier::Identifier,
    },
    context::Context,
    error::{FileCache, Result},
    lexer::Source,
    process_file,
    type_::{
        environment::{Environment, ModuleInterface, StructInterface},
        prelude::*,
    },
};
use std::{fs, path::PathBuf, sync::Arc};
use symbol_table::GlobalSymbol;

pub struct Header<'ctx> {
    context: &'ctx mut Context,
    module: ModuleInterface,
}

impl<'ctx> Header<'ctx> {
    pub fn new(context: &'ctx mut Context, module: Identifier) -> Self {
        Self {
            context,
            module: ModuleInterface::new(module),
        }
    }

    pub fn env(mut self, declarations: &[UntypedDecl]) -> Result<Environment> {
        for declaration in declarations {
            self.visit_decl(declaration)?;
        }
        let module = Arc::new(self.module);
        self.context.cache_module(module.clone());
        Ok(Environment::new(self.context.ids().clone(), module))
    }
}

impl<'a, 'ctx> DeclVisitor<'a> for Header<'ctx> {
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
        let content = FileCache::new(fs::read_to_string(&filename).expect("Expected to open file"));
        let file_id = self
            .context
            .store(filename.display().to_string(), content.clone());
        let module_name = path.last().expect("path is not empty").symbol();
        let (_, env) = process_file(self.context, module_name, file_id, content.as_ref())?;

        self.module.declare_module(env.unfold())
    }

    fn visit_struct(&mut self, structure: &'a StructDecl, _: Source) -> Self::Output {
        let mut struct_ = StructInterface::new(structure.name.clone());
        for (field_name, field_type) in &structure.fields {
            struct_.declare_associaten(field_name, field_type.clone())?;
        }
        self.module.declare_associated(
            &structure.name,
            named(
                GlobalSymbol::from(structure.name.name()),
                // TODO: add generics
                vec![],
            ),
        )?;
        self.module.declare_struct(struct_)
    }

    fn visit_function(&mut self, func: &'a FuncDecl<Self::Type>, _: Source) -> Self::Output {
        let mut arguments = Vec::new();
        for (_, type_) in &func.params {
            arguments.push(type_.clone());
        }
        self.module
            .declare_associated(&func.name, fn_(arguments, func.return_type.clone()))
    }

    fn visit_const(
        &mut self,
        name: &'a Identifier,
        type_: &'a Typed,
        _: &Expr<Self::Type>,
        _: Source,
    ) -> Self::Output {
        self.module.declare_associated(name, type_.clone())
    }
}
