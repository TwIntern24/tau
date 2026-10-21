use crate::{source_code::CSourceCode, visit_vec};
use std::{fs::File, io::prelude::*};
use tau_core::{
    ast::{
        declaration::{Decl, DeclVisitor, Function, Structure},
        expression::Expr,
        identifier::Identifier,
    },
    error,
    generator::Generator,
    typing::TypeCell,
};

pub struct CHeaderGenerator;

impl<'a> Generator<'a> for CHeaderGenerator {
    fn generate(&mut self, ast: &'a [Decl], output: &std::path::Path) -> error::Result<()> {
        let mut file = File::create(output).expect("error reading file");
        let mut path_buf = output.to_path_buf();
        path_buf.set_extension("");
        let module_name = path_buf
            .file_name()
            .expect("expect path has filename")
            .to_str()
            .expect("expect filename is UTF-8")
            .replace(".", "_")
            .replace("/", "$");
        let makro_name = module_name.to_uppercase();

        let declarations = visit_vec(ast, |x| format!("{}", self.visit_decl(x)), "\n");

        write!(
            file,
            "#ifndef {makro_name}\n#define {makro_name} 1\nnamespace {module_name} {{\n{declarations}\n}}\n#endif\n"
        ).unwrap();
        Ok(())
    }
}

impl DeclVisitor<'_> for CHeaderGenerator {
    type Output = CSourceCode;
    fn visit_import(&mut self, path: &[Identifier]) -> CSourceCode {
        let path = path
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join("/");
        CSourceCode::from(format!("#include <{path}.hpp>"))
    }

    fn visit_struct(&mut self, structure: &Structure) -> CSourceCode {
        let name = CSourceCode::from(&structure.name);
        let fields = structure
            .fields
            .iter()
            .map(|pair| {
                let (param_name, param_type) = pair;
                format!(
                    "{} {};",
                    CSourceCode::from(param_type),
                    CSourceCode::from(param_name)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("class {name} {{\npublic:\n{fields}}};").into()
    }

    fn visit_function(&mut self, function: &Function) -> CSourceCode {
        let name = CSourceCode::from(&function.name);
        let is_extern = if function.modifiers.is_extern {
            "\"C\" "
        } else {
            ""
        };
        let return_type = CSourceCode::from(function.return_type.borrow().as_ref());
        let params = visit_vec(
            &function.params,
            |x| format!("{}", CSourceCode::from(x)),
            ", ",
        );
        CSourceCode::from(format!("extern {is_extern}{return_type} {name}({params});"))
    }

    fn visit_const(&mut self, var_name: &Identifier, var_type: &TypeCell, _: &Expr) -> CSourceCode {
        let var_name = CSourceCode::from(var_name);
        let var_type = CSourceCode::from(var_type.borrow().as_ref());
        format!("extern {var_type} {var_name};").into()
    }
}
