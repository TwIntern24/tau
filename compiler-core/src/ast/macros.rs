/*#[macro_export]
macro_rules! fold_vec {
    ($folder:expr, $func:ident, $unfolded:expr) => {{
        let mut folded = Vec::with_capacity($unfolded.len());
        for e in $unfolded {
            folded.push($folder.$func(e)?);
        }
        Ok(folded)
    }};
}*/
