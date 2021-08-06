mod app;
mod codegen;

fn main() {
    let app = app::build_app().get_matches();

    let schema = app.value_of("schema").unwrap();
    let output = app.value_of("output").unwrap();

    // Parse config file
    // Parse schema
    // Create a general context
    // Render the document
    codegen::generate::generate(schema, output)
        .map_err(|e| {
            println!("Error: {:?}", e);
        })
        .unwrap();
}
