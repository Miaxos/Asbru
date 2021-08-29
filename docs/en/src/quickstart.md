# Quickstart

To create a basic schema with `Asbru`, just use:

```
asbru \
  --config example/test01/config.toml \
  --schema example/test01/schema.graphql \
  --output example/test01result/
```

This schema is a schema used to develop `Asbru`, you can access this generated schema at this address: [https://asbru-schema-01.herokuapp.com/graphql](https://asbru-schema-01.herokuapp.com/graphql).

Feel free to use [GraphQL Voyager](https://apis.guru/graphql-voyager/) to explore the generated schema.

## Docker

We provide a docker file example to create a docker image from a schema and a config file.
