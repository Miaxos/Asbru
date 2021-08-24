Connections
--------------

## Structure

Connections are `relay` specified way to navigate across lists.

```
type FriendConnection {
	"""
	Information to aid in pagination.
	"""
	pageInfo: PageInfo!
	"""
	A list of edges.
	"""
	edges: [FriendEdge]
  totalCount: Int!
}

"""
An edge in a connection.
"""
type FriendEdge {
	"""
	The item at the end of the edge
	"""
	node: Friend!
	"""
	A cursor for use in pagination
	"""
	cursor: String!
}

type Me implements User {
  ...
  friends(first: Int, last: Int, after: String, before: String, opt_parameter: Int): FriendConnection!
}
```

We want a generated code like:

```rust
#[derive(SimpleObject)]
struct FriendsFields {
  pub total_count: i32,
}

impl Me {
    pub async fn friends<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        opt_parameter: Option<i32>
    ) -> FieldResult<Connection<String, Friend, FriendsFields, EmptyFields>> {
      ...
    }
}
```

## Data Directives

We'll have multiple directives to allow us to describe how can we fetch these data.

### serviceBackedConnection
