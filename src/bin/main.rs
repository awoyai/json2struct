use json2struct::converter::*;
use serde_json::json;

fn main() {
    let json_str = json!(
        [{
            "name": "James Bond",
            "age": 33,
            "pet_phrase":[
                "Bond, James Bond.",
                "Shaken, not stirred."
            ]
        }]
    );
    let cvt = new_converter(DevLang::GOLANG);
    match cvt.generate_struct(json_str) {
        Ok(struct_str) => println!("{}", struct_str),
        Err(err) => println!("convert failed, err: {}", err),
    };    
}
