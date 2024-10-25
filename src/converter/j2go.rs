use anyhow::{anyhow, Result};
use super::Converter;

pub struct J2Go {}

impl J2Go {
    pub fn new() -> Self {
        J2Go {}
    }
}

impl Converter for J2Go {
    fn generate_struct(&self, val: serde_json::Value) -> Result<String> {
        if !val.is_object() {
            return Err(anyhow!("please consider your input is not a object"));
        }
        Ok(String::from("abc"))
    }
}
