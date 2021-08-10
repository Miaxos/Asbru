use crate::codegen::context::Context;
use crate::codegen::render::cargo::generate_cargo_toml;
use async_graphql_parser::{parse_schema, types::ServiceDocument};
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericErrors {
    #[error("IO issue while trying to access schema")]
    NotFoundError(io::Error),
    #[error("Parser error")]
    ParserError(#[from] async_graphql_parser::Error),
    #[error("Can't create the output directory")]
    CreateOutputDirectoryError(io::Error),
}

/// Open a file
fn open<P: AsRef<Path>>(path: P) -> Result<String, GenericErrors> {
    fs::read_to_string(path).map_err(GenericErrors::NotFoundError)
}

/// Parse a file with a GraphQL Parser
fn parse<S: AsRef<str>>(schema: S) -> Result<ServiceDocument, GenericErrors> {
    parse_schema(&schema).map_err(GenericErrors::ParserError)
}

pub fn generate<P: AsRef<Path>>(path: P, output: P) -> Result<(), GenericErrors> {
    let schema = open(&path).and_then(parse)?;
    let context = Context::new(&output, &schema);

    // Create a directory with src folder
    let src = output.as_ref().join(Path::new("src/"));
    fs::create_dir_all(src).map_err(GenericErrors::CreateOutputDirectoryError)?;

    // Create a Cargo.toml
    // Maybe: The Cargo.toml should be generated last, because we'll be able to describe what we are using
    // in every other files, and generate the dependencies from it.
    generate_cargo_toml(output.as_ref().join(Path::new("Cargo.toml")));

    // For each entity -> Create
    // Object type -> likely to be type in the Schema,
    // so we need to create a domain object type
    // we also need the application type
    // we should also add the directive of how it's called

    context.scalar_types();
    context.interface_types();
    Ok(())
}
