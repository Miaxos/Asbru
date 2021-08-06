use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::{collections::HashMap, path::Path};
use toml;

// TODO: Builder pattern for Cargo.

#[derive(Debug, Deserialize, Serialize)]
struct PackageConfig {
    name: String,
    version: String,
    authors: Vec<String>,
    description: Option<String>,
    readme: Option<String>,
    repository: Option<String>,
    documentation: Option<String>,
    license: Option<String>,
    edition: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BinConfig {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cargo {
    package: PackageConfig,
    bin: Option<Vec<BinConfig>>,
    dependencies: HashMap<String, String>,
}

/// Generate a Cargo toml file
/// @test-only, should create a builder pattern for a Cargo struct and a save
pub fn generate_cargo_toml<P: AsRef<Path>>(path: P) -> () {
    let package = PackageConfig {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Anthony Griffon <anthony@griffon.one>".to_string()],
        description: Some("A little description".to_string()),
        readme: None,
        documentation: None,
        repository: None,
        license: None,
        edition: Some("2018".to_string()),
    };

    let bin = BinConfig {
        name: "test".to_string(),
        path: "src/main.rs".to_string(),
    };

    let mut dependencies: HashMap<String, String> = HashMap::new();
    dependencies.insert("async-graphql".to_string(), "1.0.0".to_string());

    let cargo = Cargo {
        package,
        bin: Some(vec![bin]),
        dependencies,
    };

    let encoded = toml::to_string(&cargo).unwrap();

    let mut f = fs::File::create(&path).unwrap();
    f.write_all(encoded.as_bytes()).unwrap();
}
