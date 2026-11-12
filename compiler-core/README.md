# tau-core

[<img alt="crates.io" src="https://img.shields.io/crates/v/tau_lang_core.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tau-lang-core)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tau_lang_core-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tau-lang-core)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/quote/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/tau-lang/tau/actions?query=branch%3Amain)

This is the core compiler for the Tau language.

The Tau programming language itself is currently in active development and not ready use yet.

The compiler is modular and split into multiple components. You can replace every single component with your own or reuse these components to write your own programming language. This sub-project currently contains the following components:

- **Lexer** – Reads the tokens from an input
- **Parser** — Parses the input to an abstract syntax tree (ast)
- **Header** – Tau neither wants header files like C nor that the order of
  declaration matters like in Python. To achieve that, the compiler first
  runs the Header step that extract all declarations like type definitions
  or functions.
- **Semantic Analyzer** — Performs basic semantic analysis and type checking
  over the ast. If all checks pass, it is guaranteed that the input code is
  semantically valid.
- **Translator** – Lowers the ast to an immediate representation (ir)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md).

## License

Tau is licensed under the EUPLv1.2.

See [LICENSE](../LICENSE) for details.
