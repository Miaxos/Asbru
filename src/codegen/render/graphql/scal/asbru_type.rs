//! Absru Type trait
//! Define a specialized type which will be used to generate associated functions and definition in
//! Asbru.
//! This trait should be applied to Field
use async_graphql_parser::types::{BaseType, FieldDefinition, Type};
use thiserror::Error;

use crate::codegen::{
    context::Context,
    render::graphql::{
        fie::asbru_type::{AsbruFieldExt, AsbruFieldExtErrors},
        scalars::ToRustType,
    },
};

#[derive(Error, Debug)]
pub enum AsbruTypeErrors {
    #[error("No Connection item on a type that should have a conneciton item")]
    NoConnectionItemError,
    #[error("A connection entity should have edges")]
    NoEdgesItemError,
    #[error("Field directive errors")]
    FieldDirectivesError(#[from] AsbruFieldExtErrors),
}

pub(crate) trait AsbruType {
    /// Generate a Rust type from a GraphQL representation
    /// This function has the ability to auto import types when needed. (Soon)
    /// Some examples:
    ///
    /// String -> Option<String>
    /// [String] -> Option<Vec<Option<String>>
    /// [String!] -> Option<Vec<String>>
    ///
    /// Connections are complexe types
    /// We generate connection type from the field. Right now we do not support additional fields.
    /// A conection generated type must traverse a GraphQL Connection type to get the Node type and
    /// additional fields.
    ///
    ///
    /// We should also take into account `Field directives` associated, they'll change the internal
    /// representation of this type.
    ///
    /// ID @fromNumber -> u32
    ///
    /// You can remap the last type to what you need if u want
    fn to_gql_rust_type<'a>(&self, context: &'a Context) -> Result<String, AsbruTypeErrors>;

    /// Get the entity type without any wrapper.
    /// Usefull when you need to query the context to get the entity associated
    ///
    /// Some examples:
    ///
    /// String -> String
    /// String! -> String
    /// [String] -> String
    fn entity_type(&self) -> String;
}

struct ConnectionData {
    pub node_field: FieldDefinition,
}
/// When we create a connection, we have to generate more structure than other stuff:
/// - We need the node type
/// - We need to know if the connection is Relay-compliant
/// - We need to check the cursor type
/// - We need to check for additional fields
///
/// This function will get us these data
fn connection_data<'a>(
    context: &'a Context,
    field: &Type,
) -> Result<ConnectionData, AsbruTypeErrors> {
    let child_type = format!("{}", field.base);

    let object_types = &context.object_types();

    let connection_object = object_types
        .iter()
        .find(|x| x.doc.name.node.as_str() == &child_type)
        .ok_or(AsbruTypeErrors::NoConnectionItemError)?;

    let connection_fields = connection_object.fields();

    let edge_type = connection_fields
        .iter()
        .find(|x| x.name.node.as_str() == "edges")
        .ok_or(AsbruTypeErrors::NoEdgesItemError)?;

    let edge_type_string = edge_type.entity_type();

    let edge_object = object_types
        .iter()
        .find(|x| x.doc.name.node.as_str() == &edge_type_string)
        .ok_or(AsbruTypeErrors::NoEdgesItemError)?;

    let edge_fields = edge_object.fields();

    let node_type = edge_fields
        .iter()
        .find(|x| x.name.node.as_str() == "node")
        .ok_or(AsbruTypeErrors::NoEdgesItemError)?;

    Ok(ConnectionData {
        node_field: (*node_type).to_owned(),
    })
}

impl AsbruType for FieldDefinition {
    fn to_gql_rust_type<'a>(&self, context: &'a Context) -> Result<String, AsbruTypeErrors> {
        fn type_name<'a, S: AsRef<str>>(
            context: &'a Context,
            type_gql: &Type,
            remap_type: Option<S>,
        ) -> Result<String, AsbruTypeErrors> {
            let ret_type_name = format!("{}", type_gql);
            if ret_type_name.ends_with("Connection!") || ret_type_name.ends_with("Connection") {
                let cursor_type = "String";
                let node_type = connection_data(context, type_gql)?
                    .node_field
                    .to_gql_rust_type(context)?;

                return Ok(format!(
                    "Connection<{}, {}, EmptyFields, EmptyFields>",
                    cursor_type, node_type
                ));
            }

            let non_nullable_return = match &type_gql.base {
                BaseType::Named(name) => {
                    if let Some(remap) = remap_type {
                        remap.as_ref().to_string()
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
                BaseType::List(gql_type) => {
                    format!("Vec<{}>", type_name(context, gql_type, remap_type)?)
                }
            };

            if type_gql.nullable {
                Ok(format!("Option<{}>", non_nullable_return))
            } else {
                Ok(non_nullable_return)
            }
        }

        let return_type = type_name(
            context,
            &self.ty.node,
            None::<String>,
            // self.remap_directive()?.map(|x| x.remap_to),
        )?;

        Ok(return_type)
    }

    fn entity_type(&self) -> String {
        self.ty.node.entity_type()
    }
}
