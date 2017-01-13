use std::collections::BTreeMap;
use rustc_serialize::json::{self, Json, ToJson};

pub fn parse_str() {
    let person_json = Json::from_str( r#"{
  "id": 99,
  "name": "Hanako",
  "tag": [ "movie", "book" ]
}"# ).unwrap();

    println!("*---*---*---*---*---*");
    println!("parse_str");
    println!("*---*---*---*---*---*");

    println!("person_json: {}", person_json);

    let person = person_json.as_object().unwrap();
    for (key, value) in person.iter() {
        println!("{}: {}", key, match *value {
            Json::U64(v) => format!("{} (u64)", v),
            Json::String(ref v) => format!("{} (string)", v),
            Json::Array(ref v) => format!("{:?} (array)", v),
            _ => format!("other")
        });
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Person {
    id: u64,
    name: String,
    tag: Vec<String>
}

pub fn serialize() {
    let person = Person {
        id: 100,
        name: "Taro".to_string(),
        tag: vec![ "apple", "pineapple", "banana" ].iter().map(|&x| x.to_string()).collect::<Vec<_>>()
    };
    let encoded = json::encode( &person ).unwrap();
    let decoded: Person = json::decode( &encoded ).unwrap();

    println!("*---*---*---*---*---*");
    println!("serialize");
    println!("*---*---*---*---*---*");
    println!("encoded: {}", encoded);
    println!("decoded.id: {}", decoded.id);
    println!("decoded.name: {}", decoded.name);
    println!("decoded.tag: {:?}", decoded.tag);
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Message {
    title: String,
    body: String
}
#[derive(RustcDecodable, RustcEncodable)]
pub struct MailBox {
    account: String,
    messages: Vec<Message>
}

impl ToJson for Message {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert( "title".to_string(), self.title.to_json() );
        d.insert( "body".to_string(), self.body.to_json() );
        Json::Object(d)
    }
}

impl ToJson for MailBox {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert( "account".to_string(),  self.account.to_json() );
        d.insert( "messages".to_string(), self.messages.to_json() );
        Json::Object(d)
    }
}

pub fn custom_mapping() {
    let m1 = Message {
        title: "hello world".to_string(),
        body: "Hi. This is body.".to_string()
    };
    let m2 = Message {
        title: "Fizz Buzz".to_string(),
        body: "fizzbuzzbuzzfizz.".to_string()
    };
    let mail = MailBox {
        account: "test@example.com".to_string(),
        messages: vec![ m1, m2 ]
    };
    let json = mail.to_json();
    let text = json.to_string();

    println!("*---*---*---*---*---*");
    println!("custom_mapping");
    println!("*---*---*---*---*---*");
    println!("text: {}", text);
}
