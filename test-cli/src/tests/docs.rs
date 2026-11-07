use crate::assert_ok;
use bpaf::doc::Section;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};
use tau_cli::options::options;

const MANPAGE_FOOTER: &str = r#"
.SH DESCRIPTION
The \fBtau\fR command compiles a Tau source file and generates code using the
specified code generator (e.g., C). Generated files are written to the output
directory, with file extensions determined automatically by the code generator.

If no output directory is specified, the current working directory is used.
If no output name is specified, the input file name is used.

.SH EXAMPLES
Compile a Tau source file:

.EX
tau example/main.tau
.EE

.SH REPORTING BUGS
Report bugs at:
.UR https://codeberg.org/tau-lang/tau/issues
https://codeberg.org/tau-lang/tau/issues
.UE

.SH SEE ALSO
ochtendzon(1)

.SH COPYRIGHT
Licensed under the European Union Public Licence (EUPL).

The full license text is available at:
.UR https://codeberg.org/tau-lang/tau/src/branch/main/LICENSE
https://codeberg.org/tau-lang/tau/src/branch/main/LICENSE
.UE
"#;

// CI will ensure that documentation is always up to date
#[test]
fn update_doc_manpage() {
    let options = options();
    let mut man = options.render_manpage(
        "tau",
        Section::General,
        Some("6 Nov 2026"),
        Some("tau-lang"),
        Some("tau"),
    );
    man.push_str(MANPAGE_FOOTER);
    let mut expected_man = String::new();
    let path = dbg!(
        assert_ok!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../man/tau.1")
                .canonicalize()
        )
        .expect("path exists")
    );
    let mut file = assert_ok!(File::open(&path)).expect("could open file");
    assert_ok!(file.read_to_string(&mut expected_man)).expect("could read file");
    fs::write(path, &man).expect("could write file");

    // This will fail whenever we forgot to update the docs.
    assert_eq!(man, expected_man)
}

const MARKDOWN_HEADER: &str = r#"
# tau-cli

[<img alt="crates.io" src="https://img.shields.io/crates/v/tau_lang_cli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tau-lang-cli)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tau_lang_cli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tau-lang-cli)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/quote/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/tau-lang/tau/actions?query=branch%3Amain)

This is the default CLI for the Tau programming language.

#"#;

const MARKDOWN_FOOTER: &str = r#"
## Optional features

- `c` _(enabled by default)_ – Transpiler for Tau to C code
- `qbe` _(enabled by default)_ – Compiles the Tau ir to [QBE](https://c9x.me/compile/)
- `bright-color` – Enables bright coloring
- `dull-color` – Enables dull coloring

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md).

## License

Tau is licensed under the EUPLv1.2.

See [LICENSE](../LICENSE) for details.
"#;

#[test]
fn update_doc_readme() {
    let options = options();
    let mut markdown = String::from(MARKDOWN_HEADER);
    markdown.push_str(&options.render_markdown("Usage"));
    markdown.push_str(MARKDOWN_FOOTER);
    let mut expected_markdown = String::new();
    let path = dbg!(
        assert_ok!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../compiler-cli/README.md")
                .canonicalize()
        )
        .expect("path exists")
    );
    let mut file = assert_ok!(File::open(&path)).expect("could open file");
    assert_ok!(file.read_to_string(&mut expected_markdown)).expect("could read file");
    fs::write(path, &markdown).expect("could write file");

    // This will fail whenever we forgot to update the docs.
    assert_eq!(markdown, expected_markdown)
}
