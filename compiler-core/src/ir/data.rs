#[derive(Debug)]
pub enum DataItem {
    Symbol(String, Option<u64>),
    Str(String),
    Const(u64),
    Zero(u64),
}
