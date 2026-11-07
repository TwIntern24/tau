# tau

[<img alt="crates.io" src="https://img.shields.io/crates/v/tau_lang.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tau-lang)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/quote/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/tau-lang/tau/actions?query=branch%3Amain)

This is the default binary and compiler for the Tau language.

The Tau programming language itself is currently in active development and not ready use yet.

The compiler is modular and split into multiple components. You can replace every single component with your own or reuse these components to write your own programming language. This project currently contains the following components:

- **Tau CLI** – Default command line interface
- **Tau Core** — Pure compiler, split into Lexer, Parser, Semantic Analyzer and Translator (for ir generation). Each of these stages are again modular and can be replaced.
- **Generator C** — Transpiler for Tau to C code
- **Generator QBE** – Compiles the Tau ir to [QBE](https://c9x.me/compile/)

## Program Basics

Install the compiler:

```sh
cargo install tau-lang
```
Next, make a new source file and call it _main.tau_.

```rs
extern fn puts(v: str) -> i32;

fn main() -> i32 {
    puts("Hello World!");
    return 0;
}
```

Generate the assembly code with qbe and link it to a binary with cc:

```
$ tau main.tau
$ qbe -o main.s main.ssa && cc -o main main.s
$ ./main
Hello world!
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md).

## License

Tau is licensed under the EUPLv1.2.

See [LICENSE](../LICENSE) for details.
