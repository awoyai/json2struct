use super::{get_struct_type, underline_humb, Converter, StructType};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub struct J2Go {
    current_struct: u8,
}

impl J2Go {
    pub fn new() -> Self {
        J2Go {
            current_struct: 'A' as u8,
        }
    }
}

impl Converter for J2Go {
    fn generate_struct(&mut self, j_val: &Value) -> Result<String> {
        if !j_val.is_object() {
            return Err(anyhow!("consider your input is a object"));
        }
        let mut res = if self.current_struct != 0 {
            String::from(format!("type {} struct {{\n", self.current_struct as char))
        } else {
            "struct {\n".to_string()
        };

        self.current_struct = self.current_struct as u8 + 1;
        for (key, val) in j_val.as_object().unwrap().iter() {
            let struct_type = self.convert_struct_type(val)?;
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

    fn convert_struct_type(&mut self, j_val: &Value) -> Result<String> {
        let st = match get_struct_type(j_val) {
            StructType::BOOL => "bool",
            StructType::FLOAT64 => "float64",
            StructType::INT64 => "int64",
            StructType::NULL => "any",
            StructType::STRING => "string",
            StructType::VEC => &match j_val.as_array() {
                Some(arr) => {
                    let mut str = String::from("[]");
                    if arr.len() != 0 {
                        str.push_str(self.convert_struct_type(&arr[0])?.as_str());
                    } else {
                        str.push_str("any ");
                    }
                    str.to_owned()
                }
                None => "[]any".to_string(),
            },
            StructType::OBJECT => {
                self.current_struct = 0;
                return self.generate_struct(j_val);
            }
        };
        Ok(st.to_string())
    }
}
