use crate::codegen::{
    context::Context,
    generate::GenericErrors,
    render::{graphql::input_value_definition::InputDefinitionExt, render::Render},
};
use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::{Scope, Struct};

pub struct InputWrapper<'a> {
    // We store the whole type definition because we might need directives but it's an union, we
    // should refine this type later.
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'a>,
}

impl<'a> InputWrapper<'a> {
    fn object_name(&self) -> &'a str {
        self.doc.name.node.as_str()
    }

    fn domain_name(&self) -> String {
        format!("{}.rs", self.object_name().to_lowercase())
    }

    /// Generate an enum file for the actual type.
    /// We create a representation for each fields with no arguments and no directive.
    pub fn generate_input(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");
        scope.import("serde", "Deserialize");

        let mut object_struct = Struct::new(self.object_name());

        object_struct
            .vis("pub")
            .derive("InputObject")
            .derive("Clone");

        // Add field for it.
        let _fields = match &self.doc.kind {
            TypeKind::InputObject(input) => &input.fields,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        }
        .iter()
        .filter_map(|x| {
            x.node
                .generate_input_struct(&self.context, &mut scope, &mut object_struct);
            // If it's another entity, we should have only their getting method.
            Some((&x.node.name.node, &x.node.description))
        })
        .collect::<Vec<_>>();

        scope.push_struct(object_struct);

        self.context.create_a_new_file(
            format!("domain/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }
}

impl<'a> Render for InputWrapper<'a> {
    fn generate(&self) -> Result<(), GenericErrors> {
        self.generate_input()?;

        Ok(())
    }
}
