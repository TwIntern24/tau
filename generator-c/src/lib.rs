use crate::source_code::CSourceCode;

pub mod code;
pub mod header;
mod source_code;

pub use code::CCodeGenerator;
pub use header::CHeaderGenerator;

fn visit_vec<T, F>(params: &[T], f: F, j: &str) -> CSourceCode
where
    F: FnMut(&T) -> String,
{
    params.iter().map(f).collect::<Vec<String>>().join(j).into()
}

#[cfg(test)]
mod tests {
    use crate::lexer::Source;

    use super::*;

    #[test]
    fn patch_C_name() {
        let expected = vec![
            "this",
            "$alignas",
            "$alignof",
            "$and",
            "$and_eq",
            "$asm",
            "$atomic_cancel",
            "$atomic_commit",
            "$atomic_noexcept",
            "$auto",
            "$bitand",
            "$bitor",
            "$case",
            "$catch",
            "$char8_t",
            "$char8_t",
            "$char16_t",
            "$char32_t",
            "$class",
            "$compl",
            "$concept",
            "$consteval",
            "$constexpr",
            "$constinit",
            "$const_cast",
            "$continue",
            "$contract_assert",
            "$co_await",
            "$co_return",
            "$co_yield",
            "$decltype",
            "$default",
            "$delete",
            "$do",
            "$double",
            "$dynamic_cast",
            "$enum",
            "$explicit",
            "$export",
            "$friend",
            "$goto",
            "$inline",
            "$int",
            "$long",
            "$mutable",
            "$namespace",
            "$new",
            "$noexcept",
            "$not",
            "$not_eq",
            "$nullptr",
            "$operator",
            "$or",
            "$or_eq",
            "$private",
            "$protected",
            "$public",
            "$reflexpr",
            "$register",
            "$re",
        ]
        .iter()
        .map(|name| CSourceCode::from(name.to_string()))
        .collect::<Vec<CSourceCode>>();
        let names = vec![
            "self",
            "alignas",
            "alignof",
            "and",
            "and_eq",
            "asm",
            "atomic_cancel",
            "atomic_commit",
            "atomic_noexcept",
            "auto",
            "bitand",
            "bitor",
            "case",
            "catch",
            "char8_t",
            "char8_t",
            "char16_t",
            "char32_t",
            "class",
            "compl",
            "concept",
            "consteval",
            "constexpr",
            "constinit",
            "const_cast",
            "continue",
            "contract_assert",
            "co_await",
            "co_return",
            "co_yield",
            "decltype",
            "default",
            "delete",
            "do",
            "double",
            "dynamic_cast",
            "enum",
            "explicit",
            "export",
            "friend",
            "goto",
            "inline",
            "int",
            "long",
            "mutable",
            "namespace",
            "new",
            "noexcept",
            "not",
            "not_eq",
            "nullptr",
            "operator",
            "or",
            "or_eq",
            "private",
            "protected",
            "public",
            "reflexpr",
            "register",
            "re",
        ]
        .into_iter()
        .map(|name| {
            Identifier::from(Token::new(
                TokenType::Identifier(name.into()),
                Source::new(0, 0..0),
            ))
        });
        let values = names
            .map(|name| CSourceCode::from(&name))
            .collect::<Vec<CSourceCode>>();
        assert_eq!(values, expected);
    }
}
