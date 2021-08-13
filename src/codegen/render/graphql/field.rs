use crate::codegen::{context::Context, generate::GenericErrors, render::render::Render};
use async_graphql_parser::types::FieldDefinition;
use async_graphql_value::ConstValue;
use codegen::{Impl, Scope, Struct};
use convert_case::{Case, Casing};

use super::{
    directive::{KeyDirective, ServiceBackedQueryDirective},
    scalars::ToRustType,
};

pub trait FieldDefinitionExt {
    fn service_backed_query(&self) -> Option<ServiceBackedQueryDirective>;
    fn from_number(&self) -> bool;
    fn key_directive(&self) -> Option<KeyDirective>;
    fn is_native_gql_type(&self) -> Result<bool, GenericErrors>;

    /// Add a field to a structure
    fn generate_domain_struct(&self, domain_struct: &mut Struct) -> ();

    /// From a field, generate the associated method
    fn generate_method<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Impl,
    ) -> ();
}

impl FieldDefinitionExt for FieldDefinition {
    fn service_backed_query(&self) -> Option<ServiceBackedQueryDirective> {
        let directive = self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "serviceBackedQuery")
            .map(|x| &x.node)?;

        let method_name = match &directive.get_argument("methodName").unwrap().node {
            ConstValue::String(value) => value,
            _ => panic!("A directive is malformed"),
        }
        .to_owned();
        let service = match &directive.get_argument("service").unwrap().node {
            ConstValue::String(value) => value,
            _ => panic!("A directive is malformed"),
        }
        .to_owned();
        Some(ServiceBackedQueryDirective {
            method_name,
            service,
        })
    }

    fn key_directive(&self) -> Option<KeyDirective> {
        let directive = self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "key")
            .map(|x| &x.node)?;

        let key = match &directive.get_argument("key").unwrap().node {
            ConstValue::String(value) => value,
            _ => panic!("A directive is malformed"),
        }
        .to_owned();
        Some(KeyDirective { key })
    }

    fn from_number(&self) -> bool {
        let from_number = &self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "fromNumber")
            .map(|x| &x.node)
            .is_some();
        *from_number
    }

    fn is_native_gql_type(&self) -> Result<bool, GenericErrors> {
        self.ty.node.is_native_gql_type()
    }

    fn generate_domain_struct(&self, domain_struct: &mut Struct) -> () {
        let gql_type = &self.ty.node;
        let gql_name = &self.name.node;

        if self.from_number() {
            domain_struct.field(&format!("pub {}", gql_name), "u32");
            return;
        };

        if gql_type.is_native_gql_type().unwrap() {
            let opt_alias = self
                .key_directive()
                .map(|x| format!("#[serde(alias = \"{}\")]\n", x.key))
                .unwrap_or("".to_string());
            let type_name = &*gql_type.to_rust_type(None).unwrap();
            domain_struct.field(
                &format!("{}pub {}", &opt_alias, gql_name.to_case(Case::Snake)),
                match type_name {
                    "ID" => "String",
                    _ => type_name,
                },
            );
        } else {
            let type_name = &*gql_type.to_rust_type(Some("String")).unwrap();

            domain_struct.field(
                &format!("pub {}_id", gql_name),
                match type_name {
                    "ID" => "String",
                    _ => type_name,
                },
            );
        }
    }

    fn generate_method<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        impl_struct: &mut Impl,
    ) -> () {
        let type_object = &self.ty.node;
        let type_name = &self.name.node;

        if type_object.is_native_gql_type().unwrap() == false {
            scope.import(
                &format!(
                    "crate::domain::{}",
                    &type_object.entity_type().unwrap().to_lowercase()
                ),
                &type_object.entity_type().unwrap(),
            );
        }

        let function = impl_struct
            .new_fn(&type_name.to_case(Case::Snake))
            .set_async(true)
            .doc(
                &self
                    .description
                    .as_ref()
                    .map(|x| x.node.as_ref())
                    .unwrap_or(""),
            )
            // Scalar type
            .arg_ref_self();

        // TODO: Should add description for inputs
        for argument in self.arguments.iter() {
            function.arg(
                argument.node.name.node.as_str(),
                &argument.node.ty.node.to_rust_type(None).unwrap(),
            );
        }

        match type_object.is_native_gql_type().unwrap() {
            true => match &*type_object.to_rust_type(None).unwrap() {
                "String" => function
                    .line(format!("&self.{}", type_name.to_case(Case::Snake)))
                    .ret("&String"),
                "ID" => function
                    .line(format!(
                        "self.{}.clone().into()",
                        type_name.to_case(Case::Snake)
                    ))
                    .ret("ID"),
                _ => function
                    .line(format!("&self.{}", type_name.to_case(Case::Snake)))
                    .ret(format!("&{}", type_object.to_rust_type(None).unwrap())),
            },
            // Depending of the directives applied, should process the field/query according to it.
            // If not a query, should dataload, if query, should have a serviceBackedQuery and use it
            // to define the behaviour
            false => match self.service_backed_query() {
                Some(directive) => {
                    directive.generate_method_definition(context, &self, scope, function);
                    function
                }
                None => function.line(format!("todo!(\"WIP\")")).ret(format!(
                    "FieldResult<{}>",
                    type_object.to_rust_type(None).unwrap()
                )),
            },
        };
    }
}

impl Render for FieldDefinition {
    fn generate(&self) -> Result<(), crate::codegen::generate::GenericErrors> {
        Ok(())
    }
}
