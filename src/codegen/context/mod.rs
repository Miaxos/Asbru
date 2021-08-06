use crate::codegen::config::Config;
use async_graphql_parser::types::{
    DirectiveDefinition, SchemaDefinition, ServiceDocument, TypeDefinition, TypeKind,
    TypeSystemDefinition,
};

/// The context is like the Scope for the whole codegen, it's where we'll put every options for the
/// Codegen and every derived settings too.
/// Generators will be able to read & write inside that context to codegen their files.
/// You can view this struct as the Global Environment for Asbru
pub struct Context<'a> {
    // config: &'a Config,
    schema: &'a ServiceDocument,
}

impl<'a> Context<'a> {
    pub fn new(schema: &'a ServiceDocument) -> Self {
        Self { schema }
    }

    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.schema
            .definitions
            .iter()
            .filter_map(|type_def| match type_def {
                TypeSystemDefinition::Type(n) => Some(&n.node),
                _ => None,
            })
            .collect()
    }

    fn directive_definition(&self) -> Vec<&DirectiveDefinition> {
        self.schema
            .definitions
            .iter()
            .filter_map(|type_def| match type_def {
                TypeSystemDefinition::Directive(n) => Some(&n.node),
                _ => None,
            })
            .collect()
    }

    fn schema_definition(&self) -> Vec<&SchemaDefinition> {
        self.schema
            .definitions
            .iter()
            .filter_map(|type_def| match type_def {
                TypeSystemDefinition::Schema(n) => Some(&n.node),
                _ => None,
            })
            .collect()
    }

    pub fn scalar_types(&self) {
        self.type_definition()
            .iter()
            .map(|type_def| match type_def.kind {
                TypeKind::Scalar => {
                    println!("{:?}", type_def);
                }
                _ => {}
            })
            .collect::<Vec<_>>();
    }
}
