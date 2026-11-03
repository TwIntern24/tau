use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CodeGen {
    #[cfg(feature = "c")]
    C,
    Cranelift,
    Qbe,
}

impl Default for CodeGen {
    #[cfg(feature = "c")]
    fn default() -> Self {
        Self::C
    }

    #[cfg(not(feature = "c"))]
    fn default() -> Self {
        Self::Qbe
    }
}

impl FromStr for CodeGen {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            #[cfg(feature = "c")]
            "c" => Ok(Self::C),
            "cranelift" => Ok(Self::Cranelift),
            "qbe" => Ok(Self::Qbe),
            option => Err(format!(
                "`{option}` doesn't exist, did you mean c, cranelift or qbe?"
            )),
        }
    }
}
