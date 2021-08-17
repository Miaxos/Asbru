/// Describe different graphql types when inside a field.
/// We need to know the exact type's function to know how we have to generate the associated code.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphQLType {
    NativeType,
    EnumType,
    UnknownType,
}
