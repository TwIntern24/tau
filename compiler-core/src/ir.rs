//! Immediate representation of the code generator.
//!
//! When the ast is folded to the ir, it is expected that the ast is
//! semantically valid and all types are resolved. The Translator directly calls
//! the corresponding functions of the backend over the Emitter trait.

pub mod aggregate;
pub mod data;
pub mod emitter;
pub mod instruction;
pub mod linkage;
pub mod value;
