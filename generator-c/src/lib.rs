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
