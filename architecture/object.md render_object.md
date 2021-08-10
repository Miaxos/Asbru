# Render Object - DRAFT
-----

## Render

There are multiples strategy depending of the object we have.

### Schema related

`Schema`, `Query`, `Mutation`... We avoid creating them in the Object-type process to generate code.

These will be handled by a separate process.

### Connection / Query

We have `FieldPayload` or `EntityConnection` and so on, we should have a way to process them in a different way, so we put them appart until we can manage them.

### Simple Entity

The rest are `Simple Entities`, but it's not so simple.
For these entities, there are two files to be generated:

- domain/object_name
- application/object_name

In the `domain` we'll store simple fields, dataloaders, get data strategy...

In the `application` we'll store our GraphQL model.


First of all, we have fields in this entity, right now directives are not managed so we do not care.
