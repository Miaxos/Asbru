use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, path::Path};
use toml;

mod main;
pub use main::MainFile;

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
    #[serde(serialize_with = "toml::ser::tables_last")]
    dependencies: HashMap<String, serde_json::Value>,
}

/// Generate a Cargo toml file
/// @test-only, should create a builder pattern for a Cargo struct and a save
pub fn generate_cargo_toml<P: AsRef<Path>>(path: P) -> () {
    let package = PackageConfig {
        name: "asbru-test".to_string(),
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
        name: "asbru-test".to_string(),
        path: "src/main.rs".to_string(),
    };

    let mut dependencies: HashMap<String, serde_json::Value> = HashMap::new();
    dependencies.insert(
        "async-graphql".to_string(),
        json!({
            "version": "2.9.9",
            "features": ["url", "chrono", "apollo_tracing", "unblock", "tracing"]
        }),
    );
    dependencies.insert("async-graphql-warp".to_string(), json!("2.9.9"));
    dependencies.insert("async-graphql-derive".to_string(), json!("2.9.9"));
    dependencies.insert("async-graphql-parser".to_string(), json!("2.9.9"));

    dependencies.insert("warp".to_string(), json!("0.3.0"));

    dependencies.insert("anyhow".to_string(), json!("1.0.*"));
    dependencies.insert(
        "tokio".to_string(),
        json!({
            "version": "1",
            "features": ["full"],
        }),
    );
    dependencies.insert("serde_derive".to_string(), json!("1.0.*"));
    dependencies.insert("serde".to_string(), json!("1.0.*"));
    dependencies.insert("serde_json".to_string(), json!("1.0.*"));
    dependencies.insert(
        "tower".to_string(),
        json!({
            "version": "0.4.0",
            "features": ["full"],
        }),
    );
    dependencies.insert(
        "reqwest".to_string(),
        json!({
            "version": "0.11.*",
            "features": ["json"],
        }),
    );

    let cargo = Cargo {
        package,
        bin: Some(vec![bin]),
        dependencies,
    };

    let encoded = toml::to_string(&cargo).unwrap();

    let path = path.as_ref();
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(encoded.as_bytes()).unwrap();
    println!("Processing {:?}", &path);
}
