use super::{get_struct_type, underline_humb, Converter, StructType};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub struct J2Go {}

impl J2Go {
    pub fn new() -> Self {
        J2Go {}
    }
}

impl Converter for J2Go {
    fn generate_struct(&self, j_val: Value) -> Result<String> {
        if !j_val.is_object() {
            return Err(anyhow!("consider your input is a object"));
        }
        let mut res = String::from("type A struct {\n");
        for (key, val) in j_val.as_object().unwrap().iter() {
            let struct_type = get_struct_type(val, self.convert_struct_type);
            res.push_str(
                format!(
                    "  {} {} `json:\"{}\"`\n",
                    underline_humb(key),
                    struct_type,
                    key
                )
                .as_str(),
            );
        }
        res.push_str("}");
        Ok(res)
    }

    fn convert_struct_type(&self, struct_type: StructType) -> String {
        let st = match struct_type {
            StructType::BOOL => "bool",
            StructType::FLOAT64 => "float64",
            StructType::INT64 => "int64",
            StructType::NULL => "any",
            StructType::STRING => "string",
            StructType::VEC => "[]",
        };
        st.to_string()
    }
}
