//! # Tau Generator QBE
//!
//! Compiles the Tau ir to [QBE](https://c9x.me/compile/). This generates a code
//! in ssa form. The file still needs to be converted to assembly by invoking
//! the QBE binary and linked with the linker of your choice.
//!
//! What Tau features does this code generator support?
//!
//! - General support for functions and structs.
//! - No support for constants.
//! - Full support for control flow.
//! - Supports most expressions, however reference and dereference operators are
//!   not implemented. This also means that you cannot meaningfully use heap
//!   allocations, but stack allocations are fully supported.
//! - No support if the main function does not return an int.
//! - No name mangling.
//! - No checking of external functions. You can declare any external function,
//!   but you are solely responsible for correctly linking it.
//!
//! This project uses the QBE rust bindings. These bindings have some known bugs
//! like invalid float-point comparisons.

pub mod emitter;

pub use emitter::QbeEmitter;
