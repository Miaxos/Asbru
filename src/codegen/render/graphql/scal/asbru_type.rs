//! Absru Type trait
//! Define a specialized type which will be used to generate associated functions and definition in
//! Asbru.
//! This trait should be applied to Field
use async_graphql_parser::types::{BaseType, FieldDefinition, Type};
use codegen::{Function, Impl, Scope, Struct};
use thiserror::Error;

use crate::codegen::{
    context::Context,
    render::graphql::{
        fie::asbru_type::{AsbruFieldExt, AsbruFieldExtErrors},
        field::FieldDefinitionExt,
        inp::AsbruInputValue,
        scalars::ToRustType,
    },
};
use convert_case::{Case, Casing};

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
    /// Generate a Rust type from a GraphQL representation for a GraphQL representation
    /// This function has the ability to auto import types when needed. (Soon)
    /// Some examples:
    ///
    /// ```
    /// String -> Option<String>
    /// [String] -> Option<Vec<Option<String>>
    /// [String!] -> Option<Vec<String>>
    /// ```
    ///
    /// Connections are complexe types
    /// We generate connection type from the field. Right now we do not support additional fields.
    /// A conection generated type must traverse a GraphQL Connection type to get the Node type and
    /// additional fields.
    ///
    /// It's the function used to create the type which will define the GQL type, so no directives
    /// can alter the output type generation.
    fn to_gql_rust_type<'a>(&self, context: &'a Context) -> Result<String, AsbruTypeErrors>;

    /// Generate a Rust type from a GraphQL representation for a domain representation
    /// This function has the ability to auto import types when needed.
    /// Some examples:
    ///
    /// ```
    /// String -> Option<String>
    /// [Int] -> Option<Vec<Option<Int>>
    /// ```
    ///
    /// Depending of the Field and the associated directive, it'll create a domain representation
    /// of this type if needed.
    ///
    /// Here we'll need to apply every directives.
    /// Depending of the type kind:
    ///   - Native -> Native Representation
    ///   - Enum -> Enum Representation
    ///   - Node -> Either id or Node.
    ///
    fn struct_field_builder<'a, 'b>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &'b mut Struct,
    ) -> Result<&'b mut Struct, AsbruTypeErrors>;

    /// Generate an associated function for a GraphQL Field.
    /// This function has the ability to auto import types when needed.
    /// It'll generate the associated function for a GraphQL Field.
    ///
    /// Depending of the Field and the associated directive, it'll create the function definition.
    ///
    /// Field directives and data directives can alter this codegen.
    fn function_field_builder<'a, 'b>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        graphql_impl: &'b mut Impl,
    ) -> Result<&'b mut Impl, AsbruTypeErrors>;

    /// Get the entity type without any wrapper.
    /// Usefull when you need to query the context to get the entity associated
    ///
    /// Some examples:
    ///
    /// String -> String
    /// String! -> String
    /// [String] -> String
    fn entity_type(&self) -> String;

    /// Get the entity name
    fn name(&self) -> &str;
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

/// Generate a Rust Type from a GraphQL Type
/// Can apply a remap to alter the entity final type.
/// Should be used internally.
fn to_rust_type_name(
    context: &Context,
    type_gql: &Type,
    remap_type: Option<String>,
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
                remap
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
            let bl: String = to_rust_type_name(context, gql_type, remap_type)?;

            format!("Vec<{}>", bl)
        }
    };

    if type_gql.nullable {
        Ok(format!("Option<{}>", non_nullable_return))
    } else {
        Ok(non_nullable_return)
    }
}

/// Describe different graphql types when inside a field.
/// We need to know the exact type's function to know how we have to generate the associated code.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphQLType {
    NativeType,
    EnumType,
    ConnectionType,
    UnknownType,
}

/// Describe the GraphQL Type
fn graphql_type<'a>(type_gql: &Type, context: &'a Context) -> GraphQLType {
    match &type_gql.base {
        BaseType::Named(name) => match name.as_str() {
            "String" => GraphQLType::NativeType,
            "Bool" | "Boolean" => GraphQLType::NativeType,
            "Int" => GraphQLType::NativeType,
            "Float" => GraphQLType::NativeType,
            "ID" => GraphQLType::NativeType,
            _ => {
                if name.as_str().ends_with("Connection") {
                    GraphQLType::ConnectionType
                } else if context.is_enum(name.as_str()) {
                    GraphQLType::EnumType
                } else {
                    GraphQLType::UnknownType
                }
            }
        },
        BaseType::List(gql_type) => graphql_type(&gql_type, context),
    }
}

impl AsbruType for FieldDefinition {
    fn name(&self) -> &str {
        self.name.node.as_str()
    }

    fn to_gql_rust_type<'a>(&self, context: &'a Context) -> Result<String, AsbruTypeErrors> {
        let return_type = to_rust_type_name(
            context,
            &self.ty.node,
            None::<String>,
            // self.remap_directive()?.map(|x| x.remap_to),
        )?;

        Ok(return_type)
    }

    fn struct_field_builder<'a, 'b>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &'b mut Struct,
    ) -> Result<&'b mut Struct, AsbruTypeErrors> {
        // We check if fields have arguments because if they have, they shouldn't be store into
        // the domain, it means it's a query.
        //
        // We should also check directives associated to fields.
        // In fact we may need a field processing function for domain / application.
        let len = self.arguments.len();
        if len != 0 {
            return Ok(domain_struct);
        }

        let return_type = to_rust_type_name(
            context,
            &self.ty.node,
            self.remap_directive()?.map(|x| x.remap_to),
        )?;

        let opt_key = self
            .key_directive()?
            .map(|x| format!("#[serde(alias = \"{}\")]\n", x.key))
            .unwrap_or("".to_string());

        match graphql_type(&self.ty.node, context) {
            GraphQLType::NativeType => Ok(domain_struct.field(
                &format!("{}pub {}", &opt_key, self.name().to_case(Case::Snake)),
                match &*return_type {
                    "ID" => "String".to_string(),
                    _ => return_type,
                },
            )),
            GraphQLType::EnumType => {
                // import enum
                scope.import(
                    &format!("crate::domain::{}", self.entity_type().to_lowercase()),
                    &self.entity_type(),
                );
                Ok(domain_struct.field(
                    &format!("{}pub {}", &opt_key, self.name().to_case(Case::Snake)),
                    return_type,
                ))
            }
            // Now depending if it's a Node with a backedNode directive or not, we should maybe
            // manage things differently
            // With a backedNode, we only need an id.
            // Without a backedNode, we need to have the full model
            //
            // We do not support recursive patterns yet coz it would need a Box
            _ => {
                scope.import(
                    &format!("crate::domain::{}", self.entity_type().to_lowercase()),
                    &self.entity_type(),
                );

                Ok(domain_struct.field(
                    &format!("{}pub {}", &opt_key, self.name().to_case(Case::Snake)),
                    return_type,
                ))
                // Err(AsbruTypeErrors::UnknownError)
            }
        }
    }

    fn function_field_builder<'a, 'b>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        graphql_impl: &'b mut Impl,
    ) -> Result<&'b mut Impl, AsbruTypeErrors> {
        // Multiple behaviours possible:
        // - It's a NativeType
        // - It's an EnumType
        // - It's a Query with associated directives.

        let return_type = to_rust_type_name(context, &self.ty.node, None)?;

        let mut resolver_fct = Function::new(&self.name().to_case(Case::Snake));
        resolver_fct
            .vis("pub")
            .set_async(true)
            .doc(
                &self
                    .description
                    .as_ref()
                    .map(|x| x.node.as_ref())
                    .unwrap_or(""),
            )
            .arg_ref_self();

        for argument in self.arguments.iter() {
            resolver_fct.arg(
                &format!(
                    "{} {}",
                    argument.node.formatted_macro(),
                    argument.node.name.node.as_str()
                ),
                &to_rust_type_name(context, &argument.node.ty.node, None)?,
            );
        }

        let _ = match graphql_type(&self.ty.node, context) {
            GraphQLType::NativeType => {
                match &*return_type {
                    "String" => resolver_fct
                        .line(format!("&self.{}", self.name().to_case(Case::Snake)))
                        .ret("&String"),
                    "ID" => resolver_fct
                        .line(format!(
                            "self.{}.clone().into()",
                            self.name().to_case(Case::Snake)
                        ))
                        .ret("ID"),
                    _ => resolver_fct
                        .line(format!("&self.{}", self.name().to_case(Case::Snake)))
                        .ret(format!("&{}", return_type)),
                };
            }
            GraphQLType::EnumType => {
                scope.import(
                    &format!("crate::domain::{}", &self.entity_type().to_lowercase()),
                    &self.entity_type(),
                );

                resolver_fct
                    .line(format!("self.{}", self.name().to_case(Case::Snake)))
                    .ret(format!("{}", return_type));
            }
            GraphQLType::ConnectionType => {
                let connection_data = connection_data(context, &self.ty.node)?;
                scope.import(
                    &format!(
                        "crate::domain::{}",
                        connection_data.node_field.entity_type().to_lowercase()
                    ),
                    &connection_data.node_field.entity_type(),
                );
                scope.import("async_graphql::connection", "*");
                resolver_fct.line("todo!(\"Connection\")").ret(return_type);
            }
            // Depending of the directives applied, should process the field/query according to it.
            // If not a query, should dataload, if query, should have a serviceBackedQuery and use it
            // to define the behaviour
            _ => {
                scope.import(
                    &format!("crate::domain::{}", &self.entity_type().to_lowercase()),
                    &self.entity_type(),
                );

                match self.service_backed_query() {
                    Some(directive) => {
                        directive.generate_method_definition(
                            context,
                            &self,
                            scope,
                            &mut resolver_fct,
                        );
                    }
                    None => {
                        resolver_fct
                            .line(format!("todo!(\"WIP\")"))
                            .ret(format!("FieldResult<{}>", return_type));
                    }
                };
            }
        };

        Ok(graphql_impl.push_fn(resolver_fct))
    }

    fn entity_type(&self) -> String {
        self.ty.node.entity_type()
    }
}
