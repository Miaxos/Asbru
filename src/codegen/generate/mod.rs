use async_graphql_parser::{parse_schema, types::ServiceDocument};
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericErrors {
    #[error("IO issue while trying to access schema")]
    NotFound(#[from] io::Error),
    #[error("Parser error")]
    ParserError(#[from] async_graphql_parser::Error),
}

/// Open a file
fn open<P: AsRef<Path>>(path: P) -> Result<String, GenericErrors> {
    fs::read_to_string(path).map_err(GenericErrors::NotFound)
}

/// Parse a file with a GraphQL Parser
fn parse<S: AsRef<str>>(schema: S) -> Result<ServiceDocument, GenericErrors> {
    parse_schema(&schema).map_err(GenericErrors::ParserError)
}

pub fn generate_form_path<P: AsRef<Path>>(path: P) {
    let schema = open(&path);
    let doc = parse_schema(&schema);
    println!("{:?}", &doc);
}
