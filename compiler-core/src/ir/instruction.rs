#[derive(Debug)]
pub enum Cmp {
    /// Less then (<)
    Lt,
    /// Less equal (<=)
    Leq,
    /// Greater then (>)
    Gt,
    /// Greater equal (>=)
    Geq,
    /// Equal
    Eq,
    /// Not equal
    Neq,
}
