Asbru - A Data-oriented Service Mesh
====

<div align="center">
  <!-- CI -->
  <img src="https://github.com/Miaxos/asbru/actions/workflows/ci.yml/badge.svg" />
  <!-- Crates version -->
  <a href="https://crates.io/crates/asbru">
    <img src="https://img.shields.io/crates/v/asbru.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Documentation -->
  <a href="https://docs.rs/asbru/">
    <img src="https://docs.rs/asbru/badge.svg?style=flat-square"
      alt="Documentation" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/asbru">
    <img src="https://img.shields.io/crates/d/asbru.svg?style=flat-square"
      alt="Download" />
  </a>
</div>
<br />
<br />

/!\ WIP: Asbru won't be usable until version 0.1.0 /!\
/!\ The target is to have a MVP right now, so the code might be ugly sometimes, it's on purpose to create a prototype first. /!\

`Asbru` is an `async_graphql` Data-oriented Service Mesh generator: it'll generate an `async_graphql` project from a schema with resolvers, dataloaders, data-transformation. It'll create a Data-oriented service mesh from your schema so you'll just have to describe your microservices with your described schema to have a Data-Oriented Service Mesh like Airbnb's.

`Asbru` will be able to send metrics to apollo studio, use dataloaders patterns, store cache data into an external datastorage, be served serverless and even more.

`Asbru` is an opensource project based on Viaduct, an Airbnb's project presented [here](https://www.youtube.com/watch?v=xxk9MWCk7cM).

- [Documentation](https://docs.rs/asbru/)

_Tested at Rust version: `rustc 1.53.0 (53cb7b09b 2021-06-17)`_

## Rendered GraphQL

The rendered code is split accros three folders following an architecture inspired by the Domain Driven Design with 3 layers:

```
| main.rs
| schema.rs
| application/
|   entity/entity.rs
|   entity/query.rs
|   entity/mutation.rs
| domain/
|   entity/entity.rs
| infrastructure/
|   http.rs
|   db.rs
|   ...
```

`infrastructure` will contains every code and definitions structuring the whole application, each files should describe a high-level API which abstract the implementation, in practise, it might be coupled with the implementation.
For instance, instead of using directly `reqwest` to do http call, we create a higher level API, which describe how to do a HTTP call, and we provide an implementation for it with `reqwest`.

`domain` will describe our domains data, which called are made to call an entity, the associated dataloaders.

`application` will describe entity with a GraphQL implementation to describe query, mutations, subscriptions.

## Example

An interactive example is available with this [configuration](https://github.com/Miaxos/Asbru/tree/main/example/test01).

Available schema here:

- [https://asbru-schema-01.herokuapp.com/graphql](https://asbru-schema-01.herokuapp.com/)

## TODO

* ❌ Manage a configuration files.
* ❌ Codegen a `async_graphql` layout
  - ✅ Object generation layout
  - ✅ Modfiles
  - ✅ Cargo
  - ✅ Rust types on GraphQL scalars
* ❌ Architecture & Connect to services with a directive (at Airbnb it's something like `@serviceBackedNode`).
(...)


## Crate features

No features right now, but soon.

## References

* [Viaduct presentation](https://www.youtube.com/watch?v=xxk9MWCk7cM)
* [GraphQL](https://graphql.org)
* [Async Graphql Crates](https://github.com/async-graphql/async-graphql)
* [Codegen for async_graphql](https://github.com/atsuhiro/codegen-for-async-graphql)
