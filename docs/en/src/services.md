# Services

When you create a `Asbru` project you have to describe how your data is going to be fetched. There are multiple transports possible.

Your services should be describe inside a `config.toml` file which will gather your whole project configuration.

## Transports

### HTTP

An example of a service definition for an HTTP service:
```toml
[services]

# Beer API Based on Open Brewery DB
# https://www.openbrewerydb.org/documentation/01-listbreweries
# A Good use case to work with Connection and pagination with the list
# https://api.openbrewerydb.org/breweries
# [services.beers]
# transport = { HTTP = { endpoint = "https://api.openbrewerydb.org/breweries", method = "GET" } }

# Pets store API
# Usefull for test
[services.pets.transport]
type = "HTTP"

[services.pets.transport.info]
endpoint = "https://petstore3.swagger.io/api/v3/"

[services.pets.transport.info.method.petGetById]
route = "pet/{id}"
http_method = "GET"

[services.pets.transport.info.method.placeOrderForAPet]
route = "store/order"
http_method = "POST"
body_args = ["id", "petId", "quantity", "shipDate", "status", "complete"]
```

## Generation

Every service definition won't generate anything until you use a `fetch directive` associated.
