pub mod generated {
    use basic::generate;
    generate!("basic/fixtures/person.json");
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename="type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

use std::collections::HashMap;

// use generated::*;
use serde::{Serialize, Deserialize};

fn main(){
    let schema: Schema = serde_json::from_str(include_str!("../fixtures/person.json")).unwrap();
    println!("{:?}", schema);
}