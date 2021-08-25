# Directives
--------------

The directives describes here can alterate the codegen's generation.

## Data directives

These directives describes how a data should be fetched.
There is a 3 way modelization of a data in `Asbru`:

  < GQL Data > --- < Internal Data > --- < External Data >

When we create a GraphQL Data, we create methods and functions:
- To get simple field from the internal data modelisation.
- To fetch an External Data type and to map this type into an Internal Data modelisation from where we use the 1 point.

### Fetch directives

Fetch directives describes how we can fetch external data.

### Data transformation

These directives will affect transformations between internal representation and external modelization.

## Field directives

### Transformation directives

These directives will affect transmations between GQL modelization and internal representation.

### @fromNumber

Will describe the internal representation of the value to a number (i32).

### key

Will describe the serialization & deserialization function of the Internal Data model.
