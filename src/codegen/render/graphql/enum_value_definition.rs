use async_graphql_parser::types::EnumValueDefinition;
use async_graphql_value::ConstValue;
use codegen::Enum;

use crate::codegen::render::graphql::directive::RenameDirective;

pub trait EnumDefinitionExt {
    /// Add a value to an enum
    fn generate_enum_value(&self, enum_struct: &mut Enum) -> ();
    /// Rename directive
    fn rename_directive(&self) -> Option<RenameDirective>;
}

impl EnumDefinitionExt for EnumValueDefinition {
    fn rename_directive(&self) -> Option<RenameDirective> {
        let directive = self
            .directives
            .iter()
            .find(|x| x.node.name.node.as_str() == "rename")
            .map(|x| &x.node)?;

        let name = match &directive.get_argument("name").unwrap().node {
            ConstValue::String(value) => value,
            _ => panic!("A directive is malformed"),
        }
        .to_owned();
        Some(RenameDirective { name })
    }

    fn generate_enum_value(&self, enum_struct: &mut Enum) -> () {
        let name = self.value.node.as_str().to_uppercase();

        let opt_alias = self
            .rename_directive()
            .map(|x| format!("#[serde(rename = \"{}\")]\n", x.name))
            .unwrap_or("".to_string());

        enum_struct.new_variant(&format!("{}{}", opt_alias, name));
    }
}
