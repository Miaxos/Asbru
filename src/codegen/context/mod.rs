use std::fs;
use std::io;
use std::io::{Read, Write};
use std::{fs::File, path::Path};

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

    /// Create a new file and add it to the crate modules.
    /// The path must be relative to src/
    /// If u want to create a file into src/domain/test.rs path must be "domain/test.rs".
    ///
    /// Create folders if they do not exists.
    pub fn create_a_new_file<P: AsRef<Path>>(
        &self,
        path: P,
        content: &[u8],
    ) -> Result<File, io::Error> {
        let output = self.directory();
        let src = output.join(Path::new("src/")).join(&path);

        if src.ends_with("/") {
            fs::create_dir_all(&src)?;
        } else {
            fs::create_dir_all(&src.parent().unwrap())?;
        }

        let mut f = fs::File::create(&src)?;
        f.write_all(&content)?;

        let relative_to_directory = src
            .strip_prefix(self.directory())
            .unwrap_or(self.directory());
        let paths: Vec<&str> = relative_to_directory.to_str().unwrap().split('/').collect();

        let mut src = output.to_path_buf();

        let path_len = paths.len();

        for (i, path) in paths.iter().enumerate() {
            if i == path_len - 1 {
                break;
            }
            src = src.join(Path::new(path));

            let mod_name = paths[i + 1].trim_end_matches(".rs");

            let file_path = src.clone().join("mod.rs");
            let should_write = match fs::read_to_string(&file_path) {
                Ok(content) => content.find(&format!("pub mod {};\n", mod_name)).is_none(),
                _ => true,
            };

            if should_write {
                let mut path_file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&file_path)?;

                path_file.write_all(format!("pub mod {};\n", mod_name).as_bytes())?;
            }
        }

        Ok(f)
    }
}
