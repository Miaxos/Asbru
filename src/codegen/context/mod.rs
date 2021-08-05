/// The context is like the Scope for the whole codegen, it's where we'll put every options for the
/// Codegen and every derived settings too.
/// Generators will be able to read & write inside that context to codegen their files.
/// You can view this struct as the Global Environment for Asbru
pub struct Context {}
