use crate::codegen::{
    context::Context,
    generate::GenericErrors,
    render::{graphql::scal::asbru_type::AsbruType, render::Render},
};
use async_graphql_parser::types::FieldDefinition;
use async_graphql_value::ConstValue;

use super::{
    directive::{KeyDirective, ServiceBackedQueryDirective},
    gql_types::GraphQLType,
    scalars::ToRustType,
};

pub trait FieldDefinitionExt {
    fn service_backed_query(&self) -> Option<ServiceBackedQueryDirective>;
    fn from_number(&self) -> bool;
    // fn key_directive(&self) -> Option<KeyDirective>;
    fn is_native_gql_type<'a>(&self, context: &'a Context) -> Result<GraphQLType, GenericErrors>;

    /*
    /// Add a field to a structure
    fn generate_domain_struct<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Struct,
    ) -> ();
    */

    /*
    /// From a field, generate the associated method
    fn generate_method<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Impl,
    ) -> ();
    */

    fn interface_field_macro(&self) -> String;
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

    /*
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
    */

    fn from_number(&self) -> bool {
        let from_number = &self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "fromNumber")
            .map(|x| &x.node)
            .is_some();
        *from_number
    }

    fn is_native_gql_type<'a>(&self, context: &'a Context) -> Result<GraphQLType, GenericErrors> {
        self.ty.node.is_native_gql_type(context)
    }

    /*
    fn generate_domain_struct<'a>(
        &self,
        context: &'a Context,
        scope: &mut Scope,
        domain_struct: &mut Struct,
    ) -> () {
        let gql_type = &self.ty.node;
        let gql_name = &self.name.node;

        if self.from_number() {
            domain_struct.field(&format!("pub {}", gql_name), "u32");
            return;
        };

        // We should check if the type is either:
        // - Native
        // - Non-Native:
        //  - Enum
        //  - Interface? (WIP)
        match gql_type.is_native_gql_type(context).unwrap() {
            GraphQLType::NativeType => {
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
            }
            GraphQLType::EnumType => {
                scope.import(
                    &format!("crate::domain::{}", &gql_type.entity_type().to_lowercase()),
                    &gql_type.entity_type(),
                );

                let opt_alias = self
                    .key_directive()
                    .map(|x| format!("#[serde(alias = \"{}\")]\n", x.key))
                    .unwrap_or("".to_string());
                let type_name = &*gql_type.to_rust_type(None).unwrap();
                domain_struct.field(
                    &format!("{}pub {}", &opt_alias, gql_name.to_case(Case::Snake)),
                    type_name,
                );
            }
            _ => {
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
    }
    */

    /*
        fn generate_method<'a>(
            &self,
            context: &'a Context,
            scope: &mut Scope,
            impl_struct: &mut Impl,
        ) -> () {
            let type_object = &self.ty.node;
            let type_name = &self.name.node;

            match type_object.is_native_gql_type(context).unwrap() {
                GraphQLType::EnumType | GraphQLType::UnknownType => {
                    scope.import(
                        &format!(
                            "crate::domain::{}",
                            &type_object.entity_type().to_lowercase()
                        ),
                        &type_object.entity_type(),
                    );
                }
                _ => {}
            }

            let function = impl_struct
                .new_fn(&type_name.to_case(Case::Snake))
                .vis("pub")
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

            match type_object.is_native_gql_type(context).unwrap() {
                GraphQLType::NativeType => match &*self.to_gql_rust_type(context).unwrap() {
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
                        .ret(format!("&{}", self.to_gql_rust_type(context).unwrap())),
                },
                GraphQLType::EnumType => function
                    .line(format!("self.{}", type_name.to_case(Case::Snake)))
                    .ret(format!("{}", type_object.to_rust_type(None).unwrap())),
                GraphQLType::ConnectionType => {
                    scope.import("async_graphql::connection", "*");
                    function
                        .line("todo!(\"Connection\")")
                        .ret(self.to_gql_rust_type(context).unwrap())
                }
                // Depending of the directives applied, should process the field/query according to it.
                // If not a query, should dataload, if query, should have a serviceBackedQuery and use it
                // to define the behaviour
                _ => match self.service_backed_query() {
                    Some(directive) => {
                        directive.generate_method_definition(context, &self, scope, function);
                        function
                    }
                    // We should check if it's an enum for instance, because if it's an enum, we should
                    // try to match the actual
                    None => function.line(format!("todo!(\"WIP\")")).ret(format!(
                        "FieldResult<{}>",
                        type_object.to_rust_type(None).unwrap()
                    )),
                },
            };
        }
    */

    fn interface_field_macro(&self) -> String {
        let gql_type = &self.ty.node;
        let gql_name = &self.name.node;

        let args = &self
            .arguments
            .iter()
            .map(|x| {
                format!(
                    "arg(name = \"{}\", type = \"{}\")",
                    x.node.name.node.as_str(),
                    x.node.ty.node.to_rust_type(None).unwrap(),
                )
            })
            .collect::<Vec<String>>();

        let args_len = args.len();
        let mut args = args.join(",");

        if args_len != 0 {
            args = format!(",{}", args);
        }

        format!(
            "field(name = \"{}\", type = \"{}\"{})",
            gql_name.as_str(),
            gql_type.to_rust_type(None).unwrap(),
            args
        )
    }
}

impl Render for FieldDefinition {
    fn generate(&self) -> Result<(), crate::codegen::generate::GenericErrors> {
        Ok(())
    }
}
