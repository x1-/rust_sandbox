extern crate rustc_serialize;

mod rustc_json;

fn main() {
    rustc_json::parse_str();
    rustc_json::serialize();
    rustc_json::custom_mapping();
}
