use std::sync::RwLock;

use crate::codegen::render::graphql::scalars::ToRustType;
use crate::codegen::{context::Context, generate::GenericErrors, render::render::Render};
use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::{Impl, Scope};

const DENYLIST: [&str; 3] = ["Query", "Mutation", "Subscription"];

pub struct ObjectWrapper<'a> {
    // We store the whole type definition because we might need directives but it's an object, we
    // should refine this type later.
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'a>,
}

impl<'a> ObjectWrapper<'a> {
    fn object_name(&self) -> &'a str {
        self.doc.name.node.as_str()
    }

    fn domain_name(&self) -> String {
        format!("{}.rs", self.object_name().to_lowercase())
    }

    /// Generate a domain file for the actual type.
    /// We create a representation for each fields with no arguments and no directive.
    pub fn generate_domain_file(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("serde", "Serialize");

        let object_struct = scope
            .new_struct(self.object_name())
            .vis("pub")
            .derive("Serialize")
            .derive("Debug")
            .derive("Clone");

        // Add field for it.
        let _fields = match &self.doc.kind {
            TypeKind::Object(object) => &object.fields,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        }
        .iter()
        .filter_map(|x| {
            // We check if fields have arguments because if they have, they shouldn't be store into
            // the domain, it means it's a query.
            //
            // We should also check directives associated to fields.
            // In fact we may need a field processing function for domain / application.
            let len = x.node.arguments.len();
            if len != 0 {
                return None;
            }
            let gql_type = &x.node.ty.node;
            let gql_name = &x.node.name.node;

            if gql_type.is_native_gql_type().unwrap() {
                let type_name = &*gql_type.to_rust_type(None).unwrap();
                object_struct.field(
                    gql_name,
                    match type_name {
                        "ID" => "String",
                        _ => type_name,
                    },
                );
            } else {
                let type_name = &*gql_type.to_rust_type(Some("String")).unwrap();

                object_struct.field(
                    &format!("{}_id", gql_name),
                    match type_name {
                        "ID" => "String",
                        _ => type_name,
                    },
                );
            }
            // If it's another entity, we should have only their getting method.
            Some((&x.node.name.node, &x.node.description))
        })
        .collect::<Vec<_>>();

        self.context.create_a_new_file(
            format!("domain/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }

    /// Generate an application file for the actual type.
    pub fn generate_application_file(&self) -> Result<(), GenericErrors> {
        let mut scope = Scope::new();
        scope.import("async_graphql", "*");
        scope.import(
            &format!("crate::domain::{}", self.object_name().to_lowercase()),
            self.object_name(),
        );

        let mut impl_struct = Impl::new(self.object_name());
        impl_struct.r#macro("#[Object]");

        // Add field for it.
        let _fields = match &self.doc.kind {
            TypeKind::Object(object) => &object.fields,
            _ => {
                return Err(GenericErrors::GenericGeneratorError);
            }
        }
        .iter()
        .map(|x| {
            // TODO Fields with args
            let len = x.node.arguments.len();
            if len != 0 {
                return None;
            }

            let type_object = &x.node.ty.node;

            if type_object.is_native_gql_type().unwrap() == false {
                scope.import(
                    &format!(
                        "crate::domain::{}",
                        &type_object.entity_type().unwrap().to_lowercase()
                    ),
                    &type_object.entity_type().unwrap(),
                );
            }

            let _function = impl_struct
                .new_fn(&x.node.name.node)
                .set_async(true)
                .doc(
                    &x.node
                        .description
                        .as_ref()
                        .map(|x| x.node.as_ref())
                        .unwrap_or(""),
                )
                // Scalar type
                .ret(format!("{}", &x.node.ty.node.to_rust_type(None).unwrap()))
                .arg_ref_self()
                .line("todo!()");
            // .field(&x.node.name.node, format!("{}", &x.node.ty.node.base));
            Some((&x.node.name.node, &x.node.description))
        })
        .collect::<Vec<_>>();

        scope.push_impl(impl_struct);

        self.context.create_a_new_file(
            format!("application/{}", &self.domain_name()),
            scope.to_string().as_bytes(),
        )?;

        Ok(())
    }
}

impl<'a> Render for ObjectWrapper<'a> {
    fn generate(&self) -> Result<(), GenericErrors> {
        let object_name = self.doc.name.node.as_str();
        // Hacky denylist.
        if DENYLIST.iter().find(|name| **name == object_name).is_some() {
            return Ok(());
        };

        // If content is connection or Payload, we do not create the normal process
        //
        if object_name.ends_with("Connection") {
            // todo
            return Ok(());
        };

        if object_name.ends_with("Payload") {
            // todo
            return Ok(());
        };

        // Two separate functions... mb, it's better to have the two together to have more context
        // ?
        self.generate_domain_file()?;
        self.generate_application_file()?;
        // println!("{:?}", self.doc.name);

        // Create files
        Err(GenericErrors::GenericGeneratorError)
    }
}
