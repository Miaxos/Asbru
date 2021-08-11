use convert_case::{Case, Casing};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::{fs::File, path::Path};

use crate::codegen::generate::GenericErrors;
use crate::codegen::{config::Config, config::Service, render::graphql::object::ObjectWrapper};
use async_graphql_parser::types::{
    DirectiveDefinition, InterfaceType, SchemaDefinition, ServiceDocument, TypeDefinition,
    TypeKind, TypeSystemDefinition,
};
use codegen::Scope;

use super::render::cargo::MainFile;

/// The context is like the Scope for the whole codegen, it's where we'll put every options for the
/// Codegen and every derived settings too.
/// Generators will be able to read & write inside that context to codegen their files.
/// You can view this struct as the Global Environment for Asbru
pub struct Context<'a> {
    config: &'a Config,
    /// Source directory for codegen
    directory: &'a Path,
    // config: &'a Config,
    schema: &'a ServiceDocument,
    main_file: RefCell<MainFile>,
}

impl<'a> Context<'a> {
    pub fn new<P: AsRef<Path>>(
        directory: &'a P,
        schema: &'a ServiceDocument,
        config: &'a Config,
    ) -> Self {
        let output = directory.as_ref();
        let main_path = output.join(Path::new("src/main.rs"));

        Self {
            config,
            directory: output,
            schema,
            main_file: RefCell::new(MainFile::new(&main_path)),
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

            if i == 0 {
                let already = self.main_file().main_scope().to_string();
                let pattern_to_write = format!("mod {};", &mod_name);

                if already.find(&pattern_to_write).is_none() {
                    self.main_file()
                        .main_scope()
                        .raw(&format!("mod {};", mod_name));
                }
                continue;
            }

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
                println!("Processing {:?}", &file_path);
            }
        }

        Ok(f)
    }

    /// Generate Service file
    /// TODO Generate the service based on the transport definition
    fn generate_service_file(
        &self,
        service_name: &str,
        service: &Service,
    ) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("reqwest", "Client");
        scope.import("serde::de", "DeserializeOwned");
        // scope.import("serde", "Deserialize");

        // let result_struct = scope.new_struct(&format!("{}Result", service_name.to_case(Case::Camel))).derive("Deserialize");
        //
        /*
                 *
        use reqwest::Client;
        use serde::{de::DeserializeOwned, Deserialize};

        #[derive(Deserialize)]
        pub struct TrucResult {
            a_changed_key: String,
        }

        pub async fn truc_service<T: DeserializeOwned>(client: &Client) {
            let bl = client
                .post("url")
                .body(
                    serde_json::json!({
                        "test": "blbl",
                    })
                    .to_string(),
                )
                .send()
                .await?
                .json::<T>()
                .await?;
        }
                 */
        let _service_fn = scope
            .new_fn(&format!("{}_service", service_name))
            .set_async(true)
            .vis("pub")
            .generic("T: DeserializeOwned")
            .arg("client", "&Client")
            .ret("anyhow::Result<T>")
            .line(format!(
                r#"
    let result = client
        .post("{endpoint}")
        .body(
            serde_json::json!({{
                "test": "blbl",
            }})
            .to_string(),
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .json::<T>()
        .await
        .map_err(|e| anyhow::anyhow!(e));
        
    result
        "#,
                endpoint = "https://truc.io"
            ));

        self.create_a_new_file(
            format!("infrastructure/{}.rs", service_name),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }

    pub fn generate_services(&self) -> Result<(), GenericErrors> {
        self.config
            .services()
            .iter()
            .map(|(service_name, service)| self.generate_service_file(&service_name, service))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    /// Write to the main_file
    pub fn main_file(&self) -> RefMut<'_, MainFile> {
        self.main_file.borrow_mut()
    }
}
