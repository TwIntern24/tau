//! # Rust Core
//!
//! This is the core compiler for the Tau language.
//!
//! The Tau programming language itself is currently in active development and
//! not ready use yet.
//!
//! The compiler is modular and split into multiple components. You can replace
//! every single component with your own or reuse these components to write your
//! own programming language. This sub-project currently contains the following
//! components:
//!
//! - **Lexer** – Reads the tokens from an input
//! - **Parser** – Parses the input to an abstract syntax tree (ast)
//! - **Header** – Tau neither wants header files like C nor that the order of
//!   declaration matters like in Python. To achieve that, the compiler first
//!   runs the Header step that extract all declarations like type definitions
//!   or functions.
//! - **Semantic Analyzer** – Performs basic semantic analysis and type checking
//!   over the ast. If all checks pass, it is guaranteed that the input code is
//!   semantically valid.
//! - **Translator** – Lowers the ast to an immediate representation (ir)
//!
//! As already noted above, Tau has two ways of representing the input code:
//!
//! - **AST** – Abstract syntax tree that directly reflects what the user wrote.
//!   This representation contains references to file ids and line numbers for
//!   error tracking and both has a typed and untyped version. Transpilers like
//!   the C code generator directly generate the code from the typed ast.
//! - **IR** – Immediate representation of the code generator. When the ast is
//!   folded to the ir, it is expected that the ast is semantically valid and
//!   all types are resolved. The Translator directly calls the corresponding
//!   functions of the backend over the Emitter trait.
//!
//! This compiler attempts to report errors as good as possible. The compiler
//! uses codespan reporting for the diagnostics.
//!
//! The type system of Tau is currently extremely strict and simple. The type
//! checker only unifies two types if they have the exact same type. For
//! example, `1 + 1.0` will raise an error, because the left side is an i64 and
//! the right side is an f64. It contains only named types, functions, arrays
//! and pointers. Additionally, the compiler supports bound and unbound type
//! variables, however they are not fully implemented yet.

use crate::{
    analyzer::SemanticAnalyzer,
    ast::{TypedDecl, identifier::Identifier},
    context::Context,
    error::{FileId, Result},
    header::Header,
    lexer::{Lexer, Source},
    parser::Parser,
    type_::environment::Environment,
};
use symbol_table::GlobalSymbol;

pub mod analyzer;
pub mod ast;
pub mod context;
pub mod error;
pub mod header;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod translator;
pub mod type_;

/// Processes a single file with the given context, name of the module, file id
/// and content. The file id is the id inside the context that is used for
/// reporting errors. This will run the lexer, parser, header, and semantic
/// analyzer in order. If any of these steps fails, the error is directly
/// reported. This returns the ast together with the environment. The
/// environment contains type information, parsed modules, etc. that were
/// discoverd while processing the given file.
pub fn process_file<'ctx>(
    context: &'ctx mut Context,
    name: GlobalSymbol,
    file_id: FileId,
    content: &str,
) -> Result<(Vec<TypedDecl>, Environment)> {
    let lexer = Lexer::new(&content, file_id);
    let ast = Parser::new(lexer).parse()?;
    let mut env =
        Header::new(context, Identifier::new(name, Source::new(file_id, 0..0))).env(&ast)?;
    let ast = SemanticAnalyzer::new(&mut env).fold_ast(ast)?;
    Ok((ast, env))
}
