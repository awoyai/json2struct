use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
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
    fn convert_struct_type(&self, struct_type: StructType) -> String;
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

fn get_struct_type(val: &Value, cst: fn(struct_type: StructType) -> String) -> String {
    let mut struct_type = String::new();
    if val.is_boolean() {
        struct_type = cst(StructType::BOOL);
    } else if val.is_f64() {
        struct_type = cst(StructType::BOOL);
    } else if val.is_i64() {
        struct_type = cst(StructType::BOOL);
    } else if val.is_object() {
        todo!()
    } else if val.is_array() {
        struct_type = match val.as_array() {
            Some(arr) => {
                let mut str = cst(StructType::BOOL);
                str.push_str(get_struct_type(&arr[0], cst).as_str());
                str
            }
            None => String::from("[]any"),
        };
    } else if val.is_null() {
        struct_type = String::from("any")
    } else if val.is_string() {
        struct_type = String::from("string")
    }
    struct_type
}
