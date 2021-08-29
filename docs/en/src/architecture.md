# Architecture

This chapter will describe how `Asbru` is working internally to create a working `async-graphql` project from your schema definitions.


## Layered architecture

`Asbru` will generate a compilable `async_graphql` project based on an opinniated **layered Architecture** *(c.f. Domain Driven Design p.68)*.

A layered Architecture is an architecture which divide an application into 3 main sub-parts:

```
  - The applicative layer
  - The domain layer
  - The infrastructure layer
```

The whole purpose of this architecture is to be able to modelize your business rules and entities and separate them from the implementation details.

*You can change a layer without having to change other layers.*

The typical structure of a software built with this architecture is that top layer is able to import sub-layers but sub-layers can't import top-layers.

### Applicative layer

The applicative layer will be responsible to modelize your GraphQL schema, it'll represent the whole generated schema and describe how `async-graphql` should get data when a field is requested.

### Domain layer

The domain layer is the core layer of this architecture, it's where your business rules are represented. It means in our case, it's where your internal data representation is represented and it's where we'll store our dataloaders modelisation.

### Infrastructure layer

The infrastructure layer will manage every connection to external services, extensions, logging, metrics and monitoring which will be used by the other layers.

### Opiniated Implementation

I'm not an expert in Domain Driven Design, so I did my implementation of it, if you want to give feedback about this, do not hesitate to participate within Github. It's an open-source project :-).

## Steps

- First of all, we check if the configuration is correct, the configuration will have multiple elements: `extensions`, `services`.
- Then we parse the whole graphql schema with `async-graphql-parser`.
- We iterate over types to create struct & methods
- It's done
