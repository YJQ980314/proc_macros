use askama::Template;
use heck::{AsPascalCase, AsSnakeCase};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use anyhow::Result;

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructsTemplate {
    structs: Vec<St>,   
}

impl StructsTemplate {
    fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(Self {
            structs: schema.into_vec_st(),
        })
    }

    pub fn render(filename: &str) -> Result<String>{
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

/// input data
#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

/// output structure
pub struct St {
    /// structure name
    name: String,
    /// a list of structure fields
    fields: Vec<Fd>,
}

pub struct Fd {
    name: String,
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

// impl From<Schema> for St {
//     fn from(_: Schema) -> Self {
//         todo!()
//     }
// }

impl Schema {
    pub fn into_vec_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_str(), v))
                    .collect();
                structs.push(St::new(p(self.title.as_ref().unwrap()), fields));
                structs
            }
            _ => panic!("Not supported yet"),
        }
    }
}

fn gen_name(first: Option<&str>, second: &str) -> String {
    p(first.unwrap_or(second))
}

fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    let name = n(k);
    match v.ty.as_str() {
        "object" => {
            let sts = v.into_vec_st();
            structs.extend(sts);
            Fd::new(name, gen_name(v.title.as_deref(), k))
            // need to create a new St, field type is the St name
        }
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "string" => Fd::new(name, "String"),
        v => panic!("Unsupported schema type: {}", v),
    }
}

// pascal case
fn p(s: &str) -> String {
    AsPascalCase(s).to_string()
}

// snake case
fn n(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const PERSON1: &str = include_str!("../fixtures/person1.json");
    const PERSON2: &str = include_str!("../fixtures/person2.json");

    #[test]
    fn schema_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();
        let mut structs = schema.into_vec_st();
        assert_eq!(structs.len(), 1);
        let st = structs.pop().unwrap();
        assert_eq!(st.name, "Person");
        assert_eq!(st.fields.len(), 2);
        let mut names = st
            .fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(&names[..], &["first_name", "last_name"]);
        assert_eq!(st.fields[0].ty, "String");
        assert_eq!(st.fields[1].ty, "String");
    }

    #[test]
    fn schema_with_nested_boject_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(PERSON2).unwrap();
        let mut structs = schema.into_vec_st();
        assert_eq!(structs.len(), 2);
    }

    #[test]
    fn schema_render_should_work(){
        let result = StructsTemplate::render("fixtures/person2.json").unwrap();
        println!("{:#?}", result);
    }
}
