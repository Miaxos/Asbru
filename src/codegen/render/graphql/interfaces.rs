use crate::codegen::context::auto_import::AutoImport;
use crate::codegen::render::graphql::field::FieldDefinitionExt;
use crate::codegen::{context::Context, generate::GenericErrors, render::render::Render};
use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::{Enum, Scope, Variant};

pub struct InterfaceWrapper<'a> {
    // We store the whole type definition because we might need directives but it's an enum, we
    // should refine this type later.
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'a>,
}

impl<'a> InterfaceWrapper<'a> {
    fn object_name(&self) -> &'a str {
        self.doc.name.node.as_str()
    }

    fn domain_name(&self) -> String {
        format!("{}.rs", self.object_name().to_lowercase())
    }

    /// Generate an interface file for the actual type.
    /// We create a representation for each fields with no arguments and no directive.
    pub fn generate_interface(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("async_graphql", "*");

        let mut enum_struct = Enum::new(&format!("{}", self.object_name()));
        enum_struct.vis("pub").derive("Interface");

        // TODO: implements
        let interface = match &self.doc.kind {
            TypeKind::Interface(interface) => interface,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        };

        println!("Interface: {:?}", &self.doc);

        let fields = interface
            .fields
            .iter()
            .map(|x| {
                let field = &x.node;
                field.interface_field_macro()
            })
            .collect::<Vec<String>>();

        if fields.len() != 0 {
            let field_macro = format!(r#"#[graphql({})]"#, fields.join(", "));

            enum_struct.r#macro(&field_macro);
        }

        self.context
            .object_types()
            .iter()
            .for_each(|x| match &x.doc.kind {
                TypeKind::Object(object) => {
                    let machin = object
                        .implements
                        .iter()
                        .find(|name| name.node.as_str() == self.object_name())
                        .is_some();

                    if machin {
                        let (path, name) = x.doc.auto_import_path().unwrap();
                        scope.import(&path, &name);

                        let mut variant = Variant::new(&name);
                        variant.tuple(&name);
                        enum_struct.push_variant(variant);
                    }
                }
                _ => unreachable!(),
            });

        scope.push_enum(enum_struct);
        self.context.create_a_new_file(
            format!("domain/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }
}

impl<'a> Render for InterfaceWrapper<'a> {
    fn generate(&self) -> Result<(), GenericErrors> {
        // let object_name = self.doc.name.node.as_str();

        self.generate_interface()?;

        Ok(())
    }
}
