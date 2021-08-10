use crate::codegen::generate::GenericErrors;

/// Render trait
/// This trait must be implemented for each structures you want to Render.
pub trait Render {
    fn generate(&self) -> Result<(), GenericErrors>;
}
