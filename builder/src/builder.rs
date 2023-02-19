// 先定义好数据结构
use quote::quote;

use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, Field, token::{Comma, Token}, DeriveInput, Data, Fields, DataStruct};

pub struct BuilderContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

        let  fields = if let Data::Struct(DataStruct{
            fields: Fields::Named(FieldNamed {
                named: ref named, ..
            }),
            ..
        }) = input.data {
            named
        } else {
            panic!("Unsupported data type");
        };

        Self {
            name, fields
        }
    }

    pub fn generate(self) -> TokenStream {
        let name = self.name;
        //  builder name: {} Builder, e.g. CommandBuilder
        let builder_name = "CommandBuilder";
        // optional fields. e.g. executable: String -> executable: Option<String>,
        let optionized_fields = vec![];
        // methods: fn executable(mut self, v: String) -> Self {self.executable = Some(v); self}
        // Command::builder().executable("hello").arggs(vec![]).envs(vec![]).finish()
        let methods = vec![];
        // assign Builder fields back to original struct fields
        // field_name: self.#field_name.take().Ok_or_else("xxx need to be set!")
        let assigns = vec![];
        let ast = quote!{
            /// Builder Structure
            #[derive(Debug, default)]
            struct #builder_name {
                #(#optionized_fields, )*
            }

            impl #builder_name {
                #(#methods)*
            }

            pub fn finish(mut self) -> Result<#name, &'static str'> {
                Ok(#name {
                    #(#assigns,)*
                })
            }

            impl #name {
                fn builder() -> #builder_name {
                    Default:default()
                }
            }
        };

        
    }
}