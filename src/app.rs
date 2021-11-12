use clap::{App, AppSettings};

use rust_by_example::*;

// read cargo env
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

pub fn run(){

    // clap app
    let app = App::new(NAME.unwrap_or("unknown"))
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("unknown"))
        .about(DESCRIPTION.unwrap_or("unknown"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands([
            hello::sub_command(),
            hello::comment::sub_command(),
            hello::print::sub_command(),
        ]);

    // clap matches
    let matches = app.get_matches();

    // match subcommand
    match matches.subcommand() {
        // rust-by-example
        Some((hello::NAME, sub_matches)) => hello::sub_handler(sub_matches),
        Some((hello::comment::NAME, sub_matches)) => hello::comment::sub_handler(sub_matches),
        Some((hello::print::NAME, sub_matches)) => hello::print::sub_handler(sub_matches),


        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}