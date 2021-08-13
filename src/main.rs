mod app;
mod codegen;

fn main() {
    let app = app::build_app().get_matches();

    let schema = app.value_of("schema").unwrap();
    let output = app.value_of("output").unwrap();
    let config = app.value_of("config").unwrap();

    // Parse config file
    // Parse schema
    // Create a general context
    // Render the document
    codegen::generate::generate(schema, output, config)
        .map_err(|e| {
            println!("Error: {:?}", e);
        })
        .unwrap();
}
