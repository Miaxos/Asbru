# Introduction


`Asbru` is an `async_graphql` Data-oriented Service Mesh generator: it'll generate an `async_graphql` project from a schema with resolvers, dataloaders, data-transformation. It'll create a Data-oriented service mesh from your schema so you'll just have to describe your microservices with your described schema to have a Data-Oriented Service Mesh like Airbnb's.

It aims to be fully compatible with the Relay GraphQL specification and most of its extensions and offers type safety and high performance thanks to the `async-graphql` crates.

## Warnings

We follow the SemVer, so he `Asbru` project won't be usable before the `0.1.0 release` and won't be stable before the `1.0.0 release`.

This `book` is still a work in progress.


## Origin

The whole project comes from [an awesome talk](https://www.youtube.com/watch?v=xxk9MWCk7cM) and a [closed-source project from Airbnb named Viaduct](https://medium.com/airbnb-engineering/taming-service-oriented-architecture-using-a-data-oriented-service-mesh-da771a841344). I suggest you to read it, it's very interesting! The following and the whole project is inspired by this article.

## What is a Data-Oriented Service Mesh?

In most entrprise, we have a lot of microservices. It can be thousands to tens of thousands microservices connected together. We have dependencies everywhere between microservices.

<Insérer image qui représente un SOA ici.>

The whole purpose of a Data-Oriented Service Mesh is to create a service, which is a service mesh, which will be able to know where the data is located and will be able to route every request we have to the instances of microservices able to handle them.

We will try to create GraphQL to create a Data-Oriented Service mesh where the whole business structure is represented so services won't have to talk to each other directly but will be able to only talk to this GraphQL endpoint.

You could even modelize your whole public API through an Asbru endpoint.

With this modelisation, the only service which know how to compose data from your services is the Asbru endpoint, so you don't have to manage every dependencies inside your microservices.

## Why do this?

I enjoy using GraphQL and Rust, I was very impressed by the work from Sunli on `async_graphql`, I wanted to play my part with the Rust / GraphQL ecosystem and I saw the incredible article which described `Viaduct` from Airbnb. It's a closed source project (at least in 2021) and I though it would be awesome to have this kind of project with `async_graphql`. So I am doing this.

## How to contribute?

You can contribute to the project by creating issues, proposing ideas, suggesting ideas, coding things, whatever you want, I'm eager to know what you think of it. If you are looking for a code of conduct to follow, please follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct). It's just common sense, and be nice ❤️.
