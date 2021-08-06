use clap::{crate_version, App, AppSettings, Arg};
use std::env::var_os;

pub fn build_app() -> App<'static, 'static> {
    let clap_color_setting = if var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let app = App::new("asbru")
        .version(crate_version!())
        .author("Anthony Griffon <anthony@griffon.one>")
        .usage("asbru --schema <path> --output <path>")
        .setting(clap_color_setting)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("schema")
                .long("schema")
                .short("s")
                .overrides_with("schema")
                .takes_value(true)
                .help("Select the schema.graphql file to generate the project from")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .overrides_with("output")
                .takes_value(true)
                .help("Create the project here")
                .required(true),
        );

    app
}
