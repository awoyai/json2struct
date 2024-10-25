use anyhow::Result;
mod j2go;
mod j2rs;

pub enum DevLang {
    GOLANG,
    RUST,
}

impl DevLang {
    pub fn from_str(str: &str) -> Self {
        match str {
            "go" => Self::GOLANG,
            "rust" => Self::RUST,
            _ => Self::GOLANG,
        }
    }
}

pub trait Converter {
    fn generate_struct(&self, val: serde_json::Value) -> Result<String>;
}

pub fn new_converter(lang: DevLang) -> impl Converter {
    match lang {
        DevLang::GOLANG => j2go::J2Go::new(),
        DevLang::RUST => todo!(),
    }
}
