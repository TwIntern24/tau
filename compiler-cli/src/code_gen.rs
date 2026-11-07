use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CodeGen {
    #[cfg(feature = "c")]
    C,
    Cranelift,
    #[cfg(feature = "qbe")]
    Qbe,
}

impl Default for CodeGen {
    #[cfg(feature = "qbe")]
    fn default() -> Self {
        Self::Qbe
    }

    #[cfg(not(feature = "qbe"))]
    fn default() -> Self {
        Self::Cranelift
    }
}

impl FromStr for CodeGen {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            #[cfg(feature = "c")]
            "c" => Ok(Self::C),
            "cranelift" => Ok(Self::Cranelift),
            #[cfg(feature = "qbe")]
            "qbe" => Ok(Self::Qbe),
            option => Err(format!(
                "`{option}` doesn't exist, did you mean c, cranelift or qbe?"
            )),
        }
    }
}
