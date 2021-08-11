use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use super::generate::GenericErrors;

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportHTTP {
    endpoint: String,
    method: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportGRPC {
    endpoint: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Transport {
    HTTP(TransportHTTP),
    GRPC(TransportGRPC),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    transport: Transport,
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

        [services.user]
        transport = { HTTP = { endpoint = "http://truc.io:9009", method = "GET" } }

        [services.truc]
        transport = { GRPC = { endpoint = "http://truc.io:8002" } }
        "#;

        let config: Result<Config, _> = toml::from_str(toml_str);

        assert_eq!(config.is_ok(), true);
    }
}
