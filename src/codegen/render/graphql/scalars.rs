use super::gql_types::GraphQLType;
use async_graphql_parser::types::{BaseType, Type};

use crate::codegen::{context::Context, generate::GenericErrors};

pub trait ToRustType {
    /// Internal function, won't check the nullability of the type
    fn type_name(&self, remap_type: Option<&str>) -> Result<String, GenericErrors>;
    /// Transform a Type to a Rust type.
    /// TODO: Should add Scope to be able to dynamicly add scalars import when needed.
    /// You can remap the last type to what you need if u want
    fn to_rust_type(&self, remap_type: Option<&str>) -> Result<String, GenericErrors>;

    /// If it's a native GQL type
    fn is_native_gql_type<'a>(&self, context: &'a Context) -> Result<GraphQLType, GenericErrors>;

    /// Get the Entity type without any wrapper.
    /// If you have Option<String> it'll give you String
    /// If you have Vec<Option<Vec<Option<Entity>>>> -> Entity
    fn entity_type(&self) -> Result<String, GenericErrors>;
}

impl ToRustType for Type {
    fn type_name(&self, remap_type: Option<&str>) -> Result<String, GenericErrors> {
        let result = match &self.base {
            BaseType::Named(name) => {
                if let Some(remap) = remap_type {
                    remap.to_string()
                } else {
                    match name.as_str() {
                        "Bool" | "Boolean" => "bool",
                        "Int" => "i32",
                        "Float" => "f64",
                        "ID" => "ID",
                        _ => name.as_str(),
                    }
                    .to_string()
                }
            }
            BaseType::List(gql_type) => format!("Vec<{}>", gql_type.to_rust_type(remap_type)?),
        };
        Ok(result)
    }

    fn entity_type(&self) -> Result<String, GenericErrors> {
        let result = match &self.base {
            BaseType::Named(name) => match name.as_str() {
                "Bool" | "Boolean" => "bool",
                "Int" => "i32",
                "Float" => "f64",
                "ID" => "ID",
                _ => name.as_str(),
            }
            .to_string(),
            BaseType::List(gql_type) => gql_type.entity_type()?,
        };
        Ok(result)
    }

    fn to_rust_type(&self, remap_type: Option<&str>) -> Result<String, GenericErrors> {
        if self.nullable {
            Ok(format!("Option<{}>", self.type_name(remap_type)?))
        } else {
            Ok(self.type_name(remap_type)?)
        }
    }

    fn is_native_gql_type<'a>(&self, context: &'a Context) -> Result<GraphQLType, GenericErrors> {
        let result = match &self.base {
            BaseType::Named(name) => match name.as_str() {
                "String" => GraphQLType::NativeType,
                "Bool" | "Boolean" => GraphQLType::NativeType,
                "Int" => GraphQLType::NativeType,
                "Float" => GraphQLType::NativeType,
                "ID" => GraphQLType::NativeType,
                _ => {
                    if context.is_enum(name.as_str()) {
                        GraphQLType::EnumType
                    } else {
                        GraphQLType::UnknownType
                    }
                }
            },
            BaseType::List(gql_type) => gql_type.is_native_gql_type(context)?,
        };
        Ok(result)
    }
}
