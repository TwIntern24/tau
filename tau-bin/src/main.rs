//! # Tau
//!
//! This is the default binary and compiler for the Tau language.
//!
//! The Tau programming language itself is currently in active development and
//! not ready use yet.
//!
//! The compiler is modular and split into multiple components. You can replace
//! every single component with your own or reuse these components to write your
//! own programming language. This project currently contains the following
//! components:
//!
//! - **Tau CLI** – Default command line interface
//! - **Tau Core** — Pure compiler, split into Lexer, Parser, Semantic Analyzer
//!   and Translator (for ir generation). Each of these stages are again modular
//!   and can be replaced.
//! - **Generator C** — Transpiler for Tau to C code
//! - **Generator QBE** – Compiles the Tau ir to [QBE](https://c9x.me/compile/)
//!
//! You can install the compiler by running:
//!
//! Install the compiler:
//!
//! ```sh
//! cargo install tau-lang
//! ```

fn main() {
    tau_cli::main()
}
