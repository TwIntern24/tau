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
