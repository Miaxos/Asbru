mod app;
mod codegen;

fn main() {
    let app = app::build_app().get_matches();
    let schema = app.value_of("schema").unwrap();

    // Parse config file
    // Parse schema
    // Create a general context
    // Render the document
    codegen::generate::generate_form_path(schema);

    println!("{:?}", &schema);
}
