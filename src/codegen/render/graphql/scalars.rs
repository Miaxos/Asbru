use async_graphql_parser::types::{BaseType, Type};

use crate::codegen::generate::GenericErrors;

pub trait ToRustType {
    /// Internal function, won't check the nullability of the type
    fn type_name(&self) -> Result<String, GenericErrors>;
    /// Transform a Type to a Rust type.
    /// TODO: Should add Scope to be able to dynamicly add scalars import when needed.
    fn to_rust_type(&self) -> Result<String, GenericErrors>;
}

impl ToRustType for Type {
    fn type_name(&self) -> Result<String, GenericErrors> {
        let result = match &self.base {
            BaseType::Named(name) => match name.as_str() {
                "Bool" | "Boolean" => "bool",
                "Int" => "i32",
                "Float" => "f64",
                "ID" => "ID",
                _ => name.as_str(),
            }
            .to_string(),
            BaseType::List(gql_type) => format!("Vec<{}>", gql_type.to_rust_type()?),
        };
        Ok(result)
    }
    fn to_rust_type(&self) -> Result<String, GenericErrors> {
        if self.nullable {
            Ok(format!("Option<{}>", self.type_name()?))
        } else {
            Ok(self.type_name()?)
        }
    }
}
