use async_graphql_parser::types::{TypeDefinition, TypeKind};
use codegen::Scope;

/// The AutoImport trait define the capability for a type to autoimport his generated code inside
/// an other codegen part.
pub trait AutoImport {
    fn auto_import_path(&self) -> Option<(String, String)>;
    fn auto_import<'a>(&self, scope: &mut Scope) -> ();
}

impl AutoImport for TypeDefinition {
    fn auto_import_path(&self) -> Option<(String, String)> {
        let name = self.name.node.as_str().to_owned();
        let path = match self.kind {
            TypeKind::Enum(_) => Some((format!("crate::domain::{}", name.to_lowercase()), name)),
            TypeKind::Object(_) => Some((format!("crate::domain::{}", name.to_lowercase()), name)),
            _ => None,
        };

        path
    }

    fn auto_import<'a>(&self, scope: &mut Scope) -> () {
        let path = self.auto_import_path();

        if let Some((path, name)) = path {
            scope.import(&path, &name);
        }
    }
}
