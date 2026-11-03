#[derive(Debug, Default)]
pub struct Linkage {
    pub exported: bool,
}

impl Linkage {
    pub fn public() -> Self {
        Self { exported: true }
    }

    pub fn private() -> Self {
        Self { exported: true }
    }
}