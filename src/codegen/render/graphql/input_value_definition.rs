use async_graphql_parser::types::InputValueDefinition;
use codegen::{Scope, Struct};
use convert_case::{Case, Casing};

use crate::codegen::context::Context;

use super::{gql_types::GraphQLType, scalars::ToRustType};

pub trait InputDefinitionExt {
    fn from_number(&self) -> bool;
    /// Add a value to an enum
    fn generate_input_struct<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Struct,
    ) -> ();
}

impl InputDefinitionExt for InputValueDefinition {
    fn from_number(&self) -> bool {
        let from_number = &self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "fromNumber")
            .map(|x| &x.node)
            .is_some();
        *from_number
    }

    fn generate_input_struct<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Struct,
    ) -> () {
        let gql_type = &self.ty.node;
        let gql_name = &self.name.node;

        if self.from_number() {
            domain_struct.field(&format!("pub {}", gql_name), "u32");
            return;
        };

        // We should check if the type is either:
        // - Native
        // - Non-Native:
        //  - Enum
        //  - Interface? (WIP)
        match gql_type.is_native_gql_type(context).unwrap() {
            GraphQLType::NativeType => {
                let type_name = &*gql_type.to_rust_type(None).unwrap();
                domain_struct.field(
                    &format!("pub {}", gql_name.to_case(Case::Snake)),
                    match type_name {
                        "ID" => "String",
                        _ => type_name,
                    },
                );
            }
            GraphQLType::EnumType => {
                scope.import(
                    &format!(
                        "crate::domain::{}",
                        &gql_type.entity_type().unwrap().to_lowercase()
                    ),
                    &gql_type.entity_type().unwrap(),
                );

                let type_name = &*gql_type.to_rust_type(None).unwrap();
                domain_struct.field(&format!("pub {}", gql_name.to_case(Case::Snake)), type_name);
            }
            _ => {
                let type_name = &*gql_type.to_rust_type(Some("String")).unwrap();

                domain_struct.field(
                    &format!("pub {}_id", gql_name),
                    match type_name {
                        "ID" => "String",
                        _ => type_name,
                    },
                );
            }
        }
    }
}
