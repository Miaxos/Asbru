# Render - DRAFT
----------------

# Render process in Asbru

First of all, thank you to [codegen-for-async-graphql](https://github.com/atsuhiro/codegen-for-async-graphql) which has been used as an inspiration and codes / ideas for the whole structure & codegen come from this.

To process a Schema with Asbru we have two things:

- The Schema.graphql
- The config.toml

## Parser

Before doing anything, we will parse the whole schema with `async-graphql-parser`. As we'll be using `async-graphql` for the GraphQL engine, let's use their parser too.

We'll compute an AST (Abstract Syntax Tree), as Stephen Schneider said, it's just a "fancy way of saying heavily nested objects".

With the formatted schema, we'll then compute objects, enums, interfaces, scalars and so one...

## Generation

We have to generate files for each of the GraphQL Object, `codegen-for-async-graphql` made a trait `Render` for each type to describe how to generate files for this type.

### First issue: generation encapsulation

The single issue I can think of right now is the encapsulation of the type: Do a type can alterate the generation of another type ? Because if it can happens, then we need to have a way to represent this pattern.
It seems to happen when we'll proceed `directives` describing how objects are fetched and transformed: We'll need to change the function implementation to each objects.

To avoid having this issue, we'll proceed directived associated to a type when we are processing a type. The issue right now is to be sure directives cannot depend or alterate another directives.

### Second issue: how to differentiate custom directives for Asbru and directives for the users like.

We do not.

### Third issue: Do we let these custom directives actives for the schema generation in the final schema ?

If possible, we remove them.

### Solution 1

We'll immitate the `codegen-for-async-graphql` solution: create a wrapper for each type which will contain the Schema node.

There will be a `Render` trait for these wrapper.

The parser will create the AST, feed it to the Context, the Context will trigger the rendering.

# Simple Codegen

## Header from a Schema ?

In a Schema, there are certains objects, which are singulars like Query, Subricption, Mutation, Schema.

We may want to apply a specific workflow for them.

```
schema {
  query: Query
  mutation: Mutation
}

type Query {
  "me: Single-line comment"
  me: Me!
  active: Bool!
}

type Mutation {
  createFriendMutation(
    input: CreateFriendMutationInput!
  ): CreateFriendMutationPayload
}
```

## How to render Scalar ?

With GraphQL you can render custom scalar types with custom validators.

````graphql
scalar Url
```

Du to the complexity, there is no plan to actually provide a way to code these scalars before codegen with asbru.

The actual plan is to integrate as many as possible custom scalars beforehand and later provide either a way to add these custom scalars into Asbru with rust code or with WASM function.

## How to Render an Object ?

An object is an entity with data represented on GraphQL.


```
type Notification {
  id: ID!
  title: String!
  subtitle: String
}
```

Should return something like this:

```
struct Notification {
  id: String,
  title: String,
  subtitle: Option<String>,
}

#[Object]
impl Notification {
  async fn id(&self) -> ID {
    self.id.into()
  }

  async fn title(&self) -> String {
    &self.title
  }

  async fn subtitle(&self) -> Option<String> {
    &self.subtitle
  }
}
```

## How to Render an Interface ?

Interface is used to abstract Object with common fields;

In `async-graphql` an interface is defined with the macro `#[derive(Interface)]` or manually.

Let's focus with the derive macro for our use cases.

```
interface User {
  id: ID!
  name: String!
}

type Friend implements User {
  id: ID!
  name: String!
}

type Me implements User {
  id: ID!
  name: String!
  email: String
}
```

Should return something like:

```

struct Me {...};
struct Friend {...};

#[Object]
impl Me {
  async fn id(&self) -> ID {
    self.id
  }

  async fn name(&self) -> String {
    &self.name
  }

  async fn email(&self) -> Option<String> {
    &self.email
  }
}

#[Object]
impl Friend { 
  async fn id(&self) -> ID {
    self.id
  }

  async fn name(&self) -> String {
    &self.name
  }
}

#[derive(User)]
#[graphql(
  field(name = "id", type = "ID"),
  field(name = "name", type = "String"),
)]
enum User {
  Me(Me),
  Friend(Friend),
}
```

References: https://async-graphql.github.io/async-graphql/en/define_interface.html

## How to render an Enum ?
## How to render an Union ?
## How to render an Input ?

# Complexe Codegen with directives

## Directives
