//! FieldExtension trait
//! This define a trait for Fields for async_graphql, it'll allow us to have codegen contextual
//! data.

use crate::codegen::render::graphql::directive::{FieldDirectives, KeyDirective, RemapDirective};
use async_graphql_parser::types::FieldDefinition;
use async_graphql_value::ConstValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsbruFieldExtErrors {
    #[error("Unkown error {0} not found")]
    UnknownError(String),
    #[error("Argument {0} is missing")]
    ArgumentMissingError(String),
    #[error("Argument {0} is with the wrong type")]
    ArgumentTypeError(String),
}

pub(crate) trait AsbruFieldExt {
    /// Get associated Field directives
    /// If there are invalid field directives, it'll result in an error.
    fn field_directives(&self) -> Result<Vec<FieldDirectives>, AsbruFieldExtErrors>;
    /// There should be only one remap directive
    fn remap_directive(&self) -> Result<Option<RemapDirective>, AsbruFieldExtErrors>;
}

impl AsbruFieldExt for FieldDefinition {
    fn field_directives(&self) -> Result<Vec<FieldDirectives>, AsbruFieldExtErrors> {
        let directive = self
            .directives
            .iter()
            .filter_map(|x| match x.node.name.node.as_str() {
                "key" => match x.node.get_argument("key").map(|x| &x.node) {
                    Some(ConstValue::String(value)) => {
                        Some(Ok(FieldDirectives::KeyDirective(KeyDirective {
                            key: value.to_owned(),
                        })))
                    }
                    None => Some(Err(AsbruFieldExtErrors::ArgumentMissingError(
                        "key".to_string(),
                    ))),
                    _ => Some(Err(AsbruFieldExtErrors::ArgumentTypeError(
                        "key".to_string(),
                    ))),
                },
                "fromNumber" => Some(Ok(FieldDirectives::RemapDirective(RemapDirective {
                    remap_to: "i32".to_string(),
                }))),
                _ => None,
            })
            .collect::<Result<Vec<FieldDirectives>, _>>();

        directive
    }

    fn remap_directive(&self) -> Result<Option<RemapDirective>, AsbruFieldExtErrors> {
        Ok(self.field_directives()?.into_iter().find_map(|x| match x {
            FieldDirectives::RemapDirective(x) => Some(x),
            _ => None,
        }))
    }
}
