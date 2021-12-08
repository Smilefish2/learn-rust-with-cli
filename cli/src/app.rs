use clap::{App, AppSettings};

use rust_by_example::*;

// read cargo env
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

pub fn run(){

    // clap cli
    let app = App::new(NAME.unwrap_or("unknown"))
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("unknown"))
        .about(DESCRIPTION.unwrap_or("unknown"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands([
            // rust-by-example:hello
            hello::sub_command(),
            hello::comment::sub_command(),
            hello::print::sub_command(),
            hello::print::print_debug::sub_command(),
            hello::print::print_display::sub_command(),
            hello::print::print_display_testcase_list::sub_command(),
            hello::print::print_fmt::sub_command(),
            // rust-by-example:primitives
            primitives::sub_command(),
            primitives::literals::sub_command(),
            primitives::tuples::sub_command(),
            primitives::array::sub_command(),
            primitives::array::sub_command(),
            custom_types::structs::sub_command(),
            custom_types::enums::sub_command(),
            custom_types::enums::enum_use::sub_command(),
            custom_types::enums::c_like::sub_command(),
            custom_types::enums::testcase_linked_list::sub_command(),
            custom_types::constants::sub_command(),
            // rust-by-example:variable_bindings
            variable_bindings::sub_command(),
            variable_bindings::mutability::sub_command(),
            variable_bindings::scope::sub_command(),
            variable_bindings::declare::sub_command(),
            // rust-by-example:types
            types::sub_command(),
            types::cast::sub_command(),
            types::literals::sub_command(),
            types::inference::sub_command(),
            types::alias::sub_command(),
            // rust-by-example:conversion
            conversion::sub_command(),
            conversion::from_into::sub_command(),
            conversion::try_from_try_into::sub_command(),
            conversion::string::sub_command(),
            // rust-by-example:expression
            expression::sub_command(),
            // rust-by-example:expression
            flow_control::sub_command(),
            flow_control::if_else::sub_command(),
            flow_control::loops::sub_command(),
            flow_control::loops::nested::sub_command(),
            flow_control::loops::returns::sub_command(),

        ]);

    // clap matches
    let matches = app.get_matches();

    // match subcommand
    match matches.subcommand() {
        // rust-by-example:hello
        Some((hello::NAME, sub_matches)) => hello::sub_handler(sub_matches),
        Some((hello::comment::NAME, sub_matches)) => hello::comment::sub_handler(sub_matches),
        Some((hello::print::NAME, sub_matches)) => hello::print::sub_handler(sub_matches),
        Some((hello::print::print_debug::NAME, sub_matches)) => hello::print::print_debug::sub_handler(sub_matches),
        Some((hello::print::print_display::NAME, sub_matches)) => hello::print::print_display::sub_handler(sub_matches),
        Some((hello::print::print_display_testcase_list::NAME, sub_matches)) => hello::print::print_display_testcase_list::sub_handler(sub_matches),
        Some((hello::print::print_fmt::NAME, sub_matches)) => hello::print::print_fmt::sub_handler(sub_matches),
        // rust-by-example:primitives
        Some((primitives::NAME, sub_matches)) => primitives::sub_handler(sub_matches),
        Some((primitives::literals::NAME, sub_matches)) => primitives::literals::sub_handler(sub_matches),
        Some((primitives::tuples::NAME, sub_matches)) => primitives::tuples::sub_handler(sub_matches),
        Some((primitives::array::NAME, sub_matches)) => primitives::array::sub_handler(sub_matches),
        // rust-by-example:custom_types
        Some((custom_types::NAME, sub_matches)) => custom_types::sub_handler(sub_matches),
        Some((custom_types::structs::NAME, sub_matches)) => custom_types::structs::sub_handler(sub_matches),
        Some((custom_types::enums::NAME, sub_matches)) => custom_types::enums::sub_handler(sub_matches),
        Some((custom_types::enums::enum_use::NAME, sub_matches)) => custom_types::enums::enum_use::sub_handler(sub_matches),
        Some((custom_types::enums::c_like::NAME, sub_matches)) => custom_types::enums::c_like::sub_handler(sub_matches),
        Some((custom_types::enums::testcase_linked_list::NAME, sub_matches)) => custom_types::enums::testcase_linked_list::sub_handler(sub_matches),
        Some((custom_types::constants::NAME, sub_matches)) => custom_types::constants::sub_handler(sub_matches),
        // rust-by-example:variable_bindings
        Some((variable_bindings::NAME, sub_matches)) => variable_bindings::sub_handler(sub_matches),
        Some((variable_bindings::mutability::NAME, sub_matches)) => variable_bindings::mutability::sub_handler(sub_matches),
        Some((variable_bindings::scope::NAME, sub_matches)) => variable_bindings::scope::sub_handler(sub_matches),
        Some((variable_bindings::declare::NAME, sub_matches)) => variable_bindings::declare::sub_handler(sub_matches),
        // rust-by-example:types
        Some((types::NAME, sub_matches)) => types::sub_handler(sub_matches),
        Some((types::cast::NAME, sub_matches)) => types::cast::sub_handler(sub_matches),
        Some((types::literals::NAME, sub_matches)) => types::literals::sub_handler(sub_matches),
        Some((types::inference::NAME, sub_matches)) => types::inference::sub_handler(sub_matches),
        Some((types::alias::NAME, sub_matches)) => types::alias::sub_handler(sub_matches),
        // rust-by-example:conversion
        Some((conversion::NAME, sub_matches)) => conversion::sub_handler(sub_matches),
        Some((conversion::from_into::NAME, sub_matches)) => conversion::from_into::sub_handler(sub_matches),
        Some((conversion::try_from_try_into::NAME, sub_matches)) => conversion::try_from_try_into::sub_handler(sub_matches),
        Some((conversion::string::NAME, sub_matches)) => conversion::string::sub_handler(sub_matches),
        // rust-by-example:expression
        Some((expression::NAME, sub_matches)) => expression::sub_handler(sub_matches),
        // rust-by-example:flow_control
        Some((flow_control::NAME, sub_matches)) => flow_control::sub_handler(sub_matches),
        Some((flow_control::if_else::NAME, sub_matches)) => flow_control::if_else::sub_handler(sub_matches),
        Some((flow_control::loops::NAME, sub_matches)) => flow_control::loops::sub_handler(sub_matches),
        Some((flow_control::loops::nested::NAME, sub_matches)) => flow_control::loops::nested::sub_handler(sub_matches),
        Some((flow_control::loops::returns::NAME, sub_matches)) => flow_control::loops::returns::sub_handler(sub_matches),

        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}