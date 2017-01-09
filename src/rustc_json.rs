extern crate rustc_serialize;
use ::rustc_serialize::json::Json;

pub fn parse_str() {
    let person_json = Json::from_str( r#"{"id": 99, "name": "Hanako"}"# ).unwrap();
    println!("data: {}", person_json);
}