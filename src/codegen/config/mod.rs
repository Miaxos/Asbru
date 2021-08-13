use super::generate::GenericErrors;
use codegen::{Function, Scope, Struct};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref RE_ARGS: Regex = Regex::new(r#"\{(.*?)\}"#).unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportHTTP {
    endpoint: String,
    method: HashMap<String, MethodHTTP>,
    // There is multiple possible call:
    // GET api/get/{id}/{param}
    //
    // Where id and param sould be params from the query args
    // POST
    //
    // REMOVE, a method is not assigned to a service.
    // method: String,
}

impl TransportHTTP {
    pub fn methods(&self) -> &HashMap<String, MethodHTTP> {
        &self.method
    }
}

impl MethodHTTP {
    /// Generate method service code function
    /// We compute the necessary arguments while creating the Function code, then we create a
    /// public struct which will describe the request Arguments and which will be used inside the
    /// application/*.rs code for queries.
    pub fn generate_method(&self, scope: &mut Scope, endpoint: &str, function_name: &str) -> () {
        let mut function = Function::new(&function_name.to_case(Case::Snake));
        let body_args_struct_name = format!("{}BodyArgs", function_name.to_case(Case::Pascal));
        let query_args_struct_name = format!("{}QueryArgs", function_name.to_case(Case::Pascal));
        let route_args_struct_name = format!("{}RouteArgs", function_name.to_case(Case::Pascal));

        scope.import("serde", "Serialize");
        scope.import("serde", "Deserialize");

        let mut body_args_struct = Struct::new(&body_args_struct_name);
        body_args_struct
            .vis("pub")
            .derive("Serialize")
            .derive("Deserialize");

        let mut query_args_struct = Struct::new(&query_args_struct_name);
        query_args_struct
            .vis("pub")
            .derive("Serialize")
            .derive("Deserialize");

        let mut route_args_struct = Struct::new(&route_args_struct_name);
        route_args_struct
            .vis("pub")
            .derive("Serialize")
            .derive("Deserialize");

        if let Some(body_args) = &self.body_args {
            let _fields = body_args
                .iter()
                .map(|x| {
                    body_args_struct.field(&format!("pub {}", x.to_case(Case::Snake)), "String");
                })
                .collect::<Vec<()>>();
        }

        let format_url = RE_ARGS
            .captures_iter(&self.route)
            .map(|x| {
                route_args_struct.field(
                    &format!("pub {}", x[1].to_string().to_case(Case::Snake)),
                    "String",
                );
                format!(
                    ", {key} = route.{key}",
                    key = x[1].to_string().to_case(Case::Snake)
                )
            })
            .collect::<Vec<String>>()
            .join("");

        let final_endpoint = format!("{}{}", endpoint, self.route);

        let client_method_codegen_line = match self.http_method {
            HTTPMethod::GET => format!(
                ".get(format!(\"{url}\"{format}))",
                url = final_endpoint,
                format = format_url
            ),
            HTTPMethod::POST => format!(".post(\"{url}\")", url = final_endpoint),
            _ => unreachable!(),
        };

        /*
        let client_method_post_body = match method {
            "POST" => format!(
                r#"
            .body(serde_json::json!({
            }))
            "#
            ),
            _ => "",
        };
        */

        function
            .set_async(true)
            .vis("pub")
            .generic("T: DeserializeOwned")
            .arg("client", "&Client")
            .arg("body", &body_args_struct_name)
            .arg("query", &query_args_struct_name)
            .arg("route", &route_args_struct_name)
            .ret("anyhow::Result<T>")
            .line(format!(
                r#"
        let result = client{endpoint}
            .body(serde_json::to_string(&body)?)
            .query(&query)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .json::<T>()
            .await
            .map_err(|e| anyhow::anyhow!(e));

        result
            "#,
                endpoint = client_method_codegen_line
            ));
        /*

            let _service_fn = scope
                .new_fn(&format!("{}_service", service_name))
                .set_async(truekk)
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
            */
        scope
            .push_struct(body_args_struct)
            .push_struct(route_args_struct)
            .push_struct(query_args_struct)
            .push_fn(function);
    }

    /// Generate an API call with a transformation function to get data from an API.
    pub fn generate_api_call(&self) -> Result<String, GenericErrors> {
        todo!()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportGRPC {
    endpoint: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "info")]
pub enum Transport {
    HTTP(TransportHTTP),
    GRPC(TransportGRPC),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    UPDATE,
    HEAD,
    OPTION,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MethodHTTP {
    route: String,
    /// Args that should go from the GQL query (or mapped over) to the body params.
    http_method: HTTPMethod,
    body_args: Option<Vec<String>>,
    /// Args that should go from the GQL query (or mapped over) to the query params.
    query_args: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    transport: Transport,
}

impl Service {
    pub fn endpoint(&self) -> &str {
        match &self.transport {
            Transport::HTTP(http) => &http.endpoint,
            Transport::GRPC(grpc) => &grpc.endpoint,
        }
    }

    pub fn methods(&self) -> &HashMap<String, MethodHTTP> {
        match &self.transport {
            Transport::HTTP(http) => http.methods(),
            Transport::GRPC(_) => todo!(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    services: HashMap<String, Service>,
}

impl Config {
    /// Get a service if this service exist or return an Error.
    pub fn get_a_service(&self, name: &str) -> Result<&Service, GenericErrors> {
        match self.services.get(name) {
            Some(service) => Ok(service),
            None => Err(GenericErrors::ServiceNotFoundError(name.to_string())),
        }
    }

    pub fn services(&self) -> &HashMap<String, Service> {
        &self.services
    }
}

mod test {
    use super::*;

    #[test]
    fn test_config_format() {
        let toml_str = r#"
        [services]

        [services.user.transport]
        type = "HTTP"

        [services.user.transport.info]
        endpoint = "http://truc.io:9009"

        [services.user.transport.info.method.test]
        route = "api/v3/testMethod"
        http_method = "GET"

        [services.user.transport.info.method.testPost]
        route = "api/v3/testMethod"
        http_method = "POST"
        body_args = ["status"]
        "#;

        let config: Result<Config, _> = toml::from_str(toml_str);

        dbg!(&config);

        assert_eq!(config.is_ok(), true);
    }
}
