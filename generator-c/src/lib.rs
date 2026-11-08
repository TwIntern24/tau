//! # Tau Generator C
//!
//! Transpiler for Tau to C code.
//!
//! What Tau features does this transpiler support?
//!
//! - Full support for functions, structs and constants.
//! - Full support for control flow and expressions.
//! - No support if the main function does not return an int.
//! - No name mangling.
//! - No checking of external functions. You can declare any external function,
//!   but you are solely responsible for correctly linking it.
//! - No checks for recursive struct declarations.
//! - Array types don't work yet.
//!
//! Please also note that the transpiler to C is only intended as an early
//! prototype. The focus of the project is the LLVM code generator.
//!
//! If you look at the git history, you may realize that this project has
//! switched between an C and C++ transpiler back and forth. We decided to write
//! an C compiler because it was easier to call into the C ABI from Tau and to
//! call into Tau from C.

use crate::source_code::CSourceCode;

pub mod code;
pub mod header;
mod source_code;

pub use code::CCodeGenerator;
pub use header::CHeaderGenerator;

fn visit_vec<I, F>(params: &[I], func: F, sep: &str) -> CSourceCode
where
    F: FnMut(&I) -> String,
{
    params
        .iter()
        .map(func)
        .collect::<Vec<String>>()
        .join(sep)
        .into()
}
