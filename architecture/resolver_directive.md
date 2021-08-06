# Directives about resolvers
------------

## How do we write a resolver directive ?

At AirBnb, in their presentation, they suggest something like:

```
type User implements Node & IReservagtionGuest
  @scope(scopes: ["viaduct: public", "viaduct:internal", "user-block"])
  @serviceBackedNode(
    service: "user-block"
    methodName: "loadUsers"
  )
  @owners(list: "airbnb/user-block")
{
  id: ID!
  firstName: String,
  lastName: String,
  email: String @key(path: "emailAddress")

  ownedListings(
    after: String
    first: Int
    before: String
    last: Int
  ): UserOwnedListingsConnection

  isHost: Boolean
    @derivedField(
      classPath: "com.airbnb.viaduct.fields.IsHostProvider"
    )
}

type UserOwnedListingsConnection implements PagedConnection
  @serviceBackedConnection(
    service: "listing-block"
    methodName: "getPaginatedListing"
    sourceIdRequestFieldName: "filters.userIds"
  )
{
  pageInfo: PageInfo!
  edges: [UserOwnedListingsEdge]
}
```

### When firstName is asked, what happens ?

Let's suppose Query1 -> Give us Users with ID
firstName is requested by client, so we run the `loadUsers` to get firstname.

Let's suppose Query2 -> Give us Users with ID and FirstName
firstName is requested by client, but we have it so `loadUsers` is not requested.

Let's suppose Query3 -> Users with Id and FirstName
firstName is requested by client, but we have it but the data isn't valid for our business, we want `loadUsers` to be executed.
=> Query3 should know if there data are valid or not, not Users

Let's suppose Query4 -> Give us Users with ID and do not give us FirstName because it's a None
firstName is requested by client, and we have it but it's a None, so we shouldn't request it again.

### Scopes

Like the preview features for Github. But if we have internal routes, it may be a good option to be able to add rules over scope, for instance, "viaduct:internal" will only accept calls for within certains IPs.


### serviceBackedNode

When an another field request users, it should fill ID, and we describe how the service mesh will be able to resolve it.


## How to define a query ?

There is no input on how Airbnb are doing it for Viaduct.

```
type Query {
  userGetById(id: ID!): User
  @serviceBackedQuery(
    service: "users"
    methodName: "userById"
  )
}
```

### If the type include a resolver and the query too, what happens ?

The resolver for the query must be executed, it'll give us some Data, depending on the directives options, the Field resolver will or won't be executed.

## Derived Fields

# Supposed directives

## serviceBackedQuery

`serviceBackedQuery` is a directive for queries. It'll describe how to fetch data to start the query hydratation.


### Arguments

- `service`: Describe the Service used to get these data. (How services are described in Asbru)
- `methodName`: Method name for the service, depending of the transport method, it can have multiple meanings (RPC for GRPC, REST endpoint for REST API)
- `cacheMethod`: Describe the behavior of sub-resolvers: 

### Cache method

- `AlwaysTryToGet`: Always execute the resolver to fetch data from ID.
- `GetWhatWeDontHave`: If the data is provided by the upperLevel ( we use it even if it's a null ?).
- `SelectionTry`: Always execute the resolver for selected items.

## serviceBackedNode
## serviceBackedConnection
