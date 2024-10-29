use json2struct::converter::*;

fn main() {
    let json_str = r#"{
            "name": "James Bond",
            "age": 33,
            "pet_phrase":{
                "name": []
            }
        }"#;
    let j_val = serde_json::from_str(json_str).unwrap();
    let mut cvt = new_converter(DevLang::GOLANG);
    match cvt.generate_struct(&j_val) {
        Ok(struct_str) => println!("{}", struct_str),
        Err(err) => println!("convert failed, err: {}", err),
    };
}
