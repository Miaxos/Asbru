use std::path::Path;

use crate::codegen::{config::Config, render::graphql::object::ObjectWrapper};
use async_graphql_parser::types::{
    DirectiveDefinition, InterfaceType, SchemaDefinition, ServiceDocument, TypeDefinition,
    TypeKind, TypeSystemDefinition,
};

/// The context is like the Scope for the whole codegen, it's where we'll put every options for the
/// Codegen and every derived settings too.
/// Generators will be able to read & write inside that context to codegen their files.
/// You can view this struct as the Global Environment for Asbru
pub struct Context<'a> {
    /// Source directory for codegen
    directory: &'a Path,
    // config: &'a Config,
    schema: &'a ServiceDocument,
}

impl<'a> Context<'a> {
    pub fn new<P: AsRef<Path>>(directory: &'a P, schema: &'a ServiceDocument) -> Self {
        Self {
            directory: directory.as_ref(),
            schema,
        }
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

    /// Schema interfaces
    pub fn interface_types(&self) -> Vec<TypeDefinition> {
        self.type_definition()
            .iter()
            .filter_map(|type_def| match type_def.kind {
                TypeKind::Interface(_) => {
                    println!("{:?}", type_def);
                    Some((*type_def).clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    /// Object types
    pub fn object_types(&self) -> Vec<ObjectWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|type_def| match type_def.kind {
                TypeKind::Object(_) => {
                    // println!("{:?}", type_def);
                    Some(ObjectWrapper {
                        doc: *type_def,
                        context: self,
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn directory(&self) -> &Path {
        self.directory
    }
}
