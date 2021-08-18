use crate::codegen::{context::Context, generate::GenericErrors, render::render::Render};
use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::{Enum, Scope, Variant};

pub struct UnionWrapper<'a> {
    // We store the whole type definition because we might need directives but it's an union, we
    // should refine this type later.
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'a>,
}

impl<'a> UnionWrapper<'a> {
    fn object_name(&self) -> &'a str {
        self.doc.name.node.as_str()
    }

    fn domain_name(&self) -> String {
        format!("{}.rs", self.object_name().to_lowercase())
    }

    /// Generate an enum file for the actual type.
    /// We create a representation for each fields with no arguments and no directive.
    pub fn generate_union_file(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");
        scope.import("serde", "Deserialize");
        scope.import("async_graphql", "*");

        let mut enum_struct = Enum::new(&format!("{}", self.object_name()));

        enum_struct
            .vis("pub")
            .r#macro("#[serde(untagged)]")
            .derive("Union")
            .derive("Serialize")
            .derive("Deserialize")
            .derive("Debug")
            .derive("Clone");

        // Add field for it.
        let _fields = match &self.doc.kind {
            TypeKind::Union(union_values) => &union_values.members,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        }
        .iter()
        .filter_map(|x| {
            let union_value = &x.node;
            println!("Union value {:?}", union_value);
            // We have to import them, but how do we know the import path ?
            self.context.import_path(union_value, &mut scope);
            let mut variant = Variant::new(union_value.as_str());
            variant.tuple(union_value.as_str());
            enum_struct.push_variant(variant);
            // enum_value.generate_enum_value(enum_struct);
            Some(())
        })
        .collect::<Vec<()>>();

        scope.push_enum(enum_struct);

        self.context.create_a_new_file(
            format!("domain/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }
}

impl<'a> Render for UnionWrapper<'a> {
    fn generate(&self) -> Result<(), GenericErrors> {
        self.generate_union_file()?;

        Ok(())
    }
}
