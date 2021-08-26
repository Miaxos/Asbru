use async_graphql_parser::types::InputValueDefinition;

pub trait AsbruInputValue {
    /// Give a formatted macro based on GraphQL value and directives.
    /// Some examples:
    ///
    /// #[grahql(desc = "Id of the object")]
    fn formatted_macro(&self) -> String;
}
impl AsbruInputValue for InputValueDefinition {
    fn formatted_macro(&self) -> String {
        // Only description right now
        if let Some(desc) = &self.description {
            format!("#[graphql(desc = \"{}\")]", desc.node)
        } else {
            format!("")
        }
    }
}
