mod json_schema;

use json_schema::StructsTemplate;
use proc_macro::TokenStream;

use crate::json_schema::get_string_literal;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() {println!(\"hello world\"); }".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    // println!("{:#?}", input);
    let filename = get_string_literal(input).unwrap();
    // println!("{}", filename);
    let result = StructsTemplate::render(&filename).unwrap();
    result.parse().unwrap()
    // let structs = todo!();
    // let result = structs.render();
    // result.parse().unwrap()
    // TokenStream::default()
}