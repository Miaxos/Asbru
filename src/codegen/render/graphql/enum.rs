use crate::codegen::render::graphql::enum_value_definition::EnumDefinitionExt;
use crate::codegen::render::graphql::field::FieldDefinitionExt;
use crate::codegen::{context::Context, generate::GenericErrors, render::render::Render};
use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::{Enum, Impl, Scope};

pub struct EnumWrapper<'a> {
    // We store the whole type definition because we might need directives but it's an enum, we
    // should refine this type later.
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'a>,
}

impl<'a> EnumWrapper<'a> {
    fn object_name(&self) -> &'a str {
        self.doc.name.node.as_str()
    }

    fn domain_name(&self) -> String {
        format!("{}.rs", self.object_name().to_lowercase())
    }

    /// Generate an enum file for the actual type.
    /// We create a representation for each fields with no arguments and no directive.
    pub fn generate_enum_file(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");
        scope.import("serde", "Deserialize");
        scope.import("async_graphql", "*");

        let enum_struct = scope
            .new_enum(&format!("{}", self.object_name()))
            .vis("pub")
            .derive("Enum")
            .derive("Debug")
            .derive("Clone")
            .derive("Deserialize")
            .derive("Serialize")
            .derive("Eq")
            .derive("PartialEq")
            .derive("Copy");

        // Add field for it.
        let _fields = match &self.doc.kind {
            TypeKind::Enum(enum_values) => &enum_values.values,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        }
        .iter()
        .filter_map(|x| {
            let enum_value = &x.node;
            enum_value.generate_enum_value(enum_struct);
            Some(())
        })
        .collect::<Vec<()>>();

        self.context.create_a_new_file(
            format!("domain/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }
}

impl<'a> Render for EnumWrapper<'a> {
    fn generate(&self) -> Result<(), GenericErrors> {
        // let object_name = self.doc.name.node.as_str();

        self.generate_enum_file()?;

        Ok(())
    }
}
