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


`Asbru` is an `async_graphql` Data-oriented Service Mesh generator: it'll generate an `async_graphql` project from a schema with resolvers, dataloaders, data-transformation. It'll create a Data-oriented service mesh from your schema so you'll just have to describe your microservices with your described schema to have a Data-Oriented Service Mesh like Airbnb's.

`Asbru` will be able to send metrics to apollo studio, use dataloaders patterns, store cache data into an external datastorage, be served serverless and even more.

`Asbru` is an opensource project based on Viaduct, an Airbnb's project presented [here](https://www.youtube.com/watch?v=xxk9MWCk7cM).

- [Documentation](https://docs.rs/asbru/)

_Tested at Rust version: `rustc 1.53.0 (53cb7b09b 2021-06-17)`_

## TODO

* [] Manage a configuration files.
* [] Codegen a `async_graphql` layout
* [] Architecture & Connect to services with a directive (at Airbnb it's something like `@serviceBackedNode`).
(...)


## Crate features

No features right now, but soon.

## References

* [Viaduct presentation](https://www.youtube.com/watch?v=xxk9MWCk7cM)
* [GraphQL](https://graphql.org)
* [Async Graphql Crates](https://github.com/async-graphql/async-graphql)
* [Codegen for async_graphql](https://github.com/atsuhiro/codegen-for-async-graphql)
