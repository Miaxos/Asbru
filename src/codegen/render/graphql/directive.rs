use async_graphql_parser::types::FieldDefinition;
use codegen::{Function, Scope};
use convert_case::{Case, Casing};

use crate::codegen::{context::Context, render::graphql::scalars::ToRustType};

pub struct ServiceBackedQueryDirective {
    pub method_name: String,
    pub service: String,
}

impl ServiceBackedQueryDirective {
    pub fn generate_method_definition<'a>(
        &self,
        context: &'a Context,
        field: &FieldDefinition,
        scope: &mut Scope,
        function: &mut Function,
    ) -> () {
        let main_name = format!("{}_{}", self.service, self.method_name);
        let method_name = format!(
            "{}_{}_method",
            self.service,
            self.method_name.to_case(Case::Snake)
        );
        let body_args = format!("{}MethodBodyArgs", main_name).to_case(Case::Pascal);
        let route_args =
            format!("{}_{}MethodRouteArgs", self.service, self.method_name).to_case(Case::Pascal);
        let query_args =
            format!("{}_{}MethodQueryArgs", self.service, self.method_name).to_case(Case::Pascal);
        scope.import(
            &format!("crate::infrastructure::{}", self.service),
            &method_name,
        );
        scope.import(
            &format!("crate::infrastructure::{}", self.service),
            &body_args,
        );
        scope.import(
            &format!("crate::infrastructure::{}", self.service),
            &route_args,
        );
        scope.import(
            &format!("crate::infrastructure::{}", self.service),
            &query_args,
        );

        let method = context
            .get_service_by_name(&self.service)
            .unwrap()
            .methods()
            .get(&self.method_name)
            .unwrap();

        let route_method_construct =
            method.route_method_construct(&main_name.to_case(Case::Pascal));
        let body_method_construct = method.body_method_construct(&main_name.to_case(Case::Pascal));
        let query_method_construct =
            method.query_method_construct(&main_name.to_case(Case::Pascal));

        function.line(&format!(
            r#"
    let client = reqwest::Client::new();
    let result = {method}::<{method_type}>(
        &client,
        {body},
        {query},
        {route}
        )
        .await
        .map_err(|e| {{
        println!("error: {{:?}}", e);
        "An error roccured while querying the data from the backend service."
        }})?;

        Ok(result)
                "#,
            method = method_name,
            method_type = &field.ty.node.to_rust_type(None).unwrap(),
            route = route_method_construct,
            body = body_method_construct,
            query = query_method_construct
        ));

        function.ret(format!(
            "FieldResult<{}>",
            field.ty.node.to_rust_type(None).unwrap()
        ));
    }
}
pub struct KeyDirective {
    pub key: String,
}
