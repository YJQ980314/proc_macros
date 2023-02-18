pub mod generated {
    use basic::generate;
    generate!("basic/fixtures/person2.json");
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename="type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

use std::collections::HashMap;

use generated::*;

use serde::{Serialize, Deserialize};

// fn main(){
//     let schema: Schema = serde_json::from_str(include_str!("../fixtures/person.json")).unwrap();
//     println!("{:?}", schema);
// }

fn main() {
    let schema: Schema = serde_json::from_str(include_str!("../fixtures/person.json")).unwrap();
    println!("{:?}", schema);
    let person = Person {
        first_name: "JQ".into(),
        last_name: "Y".into(),
        skill: Skill {
            name: "Rust".into(),
        },
    };

    println!("{:#?}", person);
}