# Service
-----------

When you describe how a query should fetch their data, you use a service.

## What is a service ?

A transport method to connect a service.

## Should we describe methods inside toml?

method (for http):

// GET EXAMPLE
route: "api/machin/{id}"
method: "GET"
body: null

// POST EXAMPLE
route "api/truc/add/{id}"
mathod: "POST"
body: {
  machin: {truc},
}

## Configuration

Here is an example:
````toml
[services]

[services.user.transport]
type = "HTTP"

[services.user.transport.info]
endpoint = "http://truc.io:9009"

[services.user.transport.info.method.test]
route = "api/v3/testMethod"
http_method = "GET"
```

## Method

A method is a definition to an API route.

For instance, when you define a HTTP Method, `Absru` will generate an associated function to call this HTTP Route.

### Arguments

An HTTP route can have multiple arguments, either within the body or within the url.

```
GET - https://instance.tld/api/pet/{id}
GET - https://instance.tld/api/search?{arg1}={val1}&{arg2}={val2}
POST - https://instance.tld/api/pet/{id}
  | {
  |   "method": {method_name},
  |   "new_owner_id": {owner_id},
  | }
```

To know how to call these API, we need to define how what we need to call them and define where we use these arguments.

`Asbru` will be able to automatically fill these arguments when these methods are used with `@serviceBackedQuery` and the GraphQL arguments matches the methods arguments.

Or you'll have to map these arguments within `@serviceBackedQuery`.

`@serviceBackedNode` will only accept methods with only one arguments: `id`.

````toml
[services]

[services.user.transport]
type = "HTTP"

[services.user.transport.info]
endpoint = "http://truc.io:9009"

[services.user.transport.info.method.getAPetByID]
route = "api/v3/pet/{id}"
http_method = "GET"

[services.user.transport.info.method.search]
route = "api/v3/search"
http_method = "GET"
[services.user.transport.info.method.search.query_params]
TODO

[services.user.transport.info.method.createANewPet]
route = "api/v3/pet/create"
http_method = "POST"
body_args = ["method", "name", "owner_id", "something"]
```
