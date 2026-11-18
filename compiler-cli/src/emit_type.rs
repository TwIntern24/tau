use std::str::FromStr;

/// Types of files the compiler emits.
#[derive(Debug, Clone, Default)]
pub enum EmitType {
    LlvmIr,
    #[default]
    Obj,
}

impl FromStr for EmitType {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "llvm-ir" => Ok(Self::LlvmIr),
            "obj" => Ok(Self::Obj),
            option => Err(format!(
                "`{option}` doesn't exist, did you mean llvm-ir or obj?"
            )),
        }
    }
}
