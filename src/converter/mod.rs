use anyhow::Result;
use serde_json::Value;
mod j2go;
mod j2rs;

pub enum DevLang {
    GOLANG,
    RUST,
}

pub enum StructType {
    BOOL,
    FLOAT64,
    INT64,
    NULL,
    STRING,
    VEC,
    OBJECT,
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
    fn generate_struct(&mut self, val: &Value) -> Result<String>;
    fn convert_struct_type(&mut self, val: &Value) -> Result<String>;
}

pub fn new_converter(lang: DevLang) -> impl Converter {
    match lang {
        DevLang::GOLANG => j2go::J2Go::new(),
        DevLang::RUST => todo!(),
    }
}

pub fn underline_humb(s: &String) -> String {
    let mut humb_char_vec = Vec::new();
    let mut underline_flag = false;
    for (i, item) in s.as_bytes().iter().enumerate() {
        let mut char = char::from(*item);
        if char as u8 == '_' as u8 {
            underline_flag = true;
            continue;
        }
        if i == 0 || underline_flag {
            char = char.to_ascii_uppercase();
            underline_flag = false
        }
        humb_char_vec.push(char);
    }
    humb_char_vec.iter().collect::<String>()
}

fn get_struct_type(val: &Value) -> StructType {
    if val.is_boolean() {
        return StructType::BOOL;
    } else if val.is_f64() {
        return StructType::FLOAT64;
    } else if val.is_i64() {
        return StructType::INT64;
    } else if val.is_object() {
        return StructType::OBJECT;
    } else if val.is_array() {
        return StructType::VEC;
    } else if val.is_string() {
        return StructType::STRING;
    }
    StructType::NULL
}
