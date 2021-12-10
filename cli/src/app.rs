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
            flow_control::whiles::sub_command(),
            flow_control::fors::sub_command(),
            flow_control::matchs::sub_command(),
            flow_control::matchs::destructuring::destructure_tuple::sub_command(),
            flow_control::matchs::destructuring::destructure_enum::sub_command(),
            flow_control::matchs::destructuring::destructure_pointers::sub_command(),
            flow_control::matchs::destructuring::destructure_structures::sub_command(),
            flow_control::matchs::guard::sub_command(),
            flow_control::matchs::binding::sub_command(),
            flow_control::if_let::sub_command(),
            flow_control::while_let::sub_command(),
            // rust-by-example:expression
            functions::sub_command(),
            functions::methods::sub_command(),
            functions::closures::sub_command(),
            functions::closures::capture::sub_command(),
            functions::closures::input_parameters::sub_command(),
            functions::closures::anonymity::sub_command(),
            functions::closures::input_functions::sub_command(),
            functions::closures::output_parameters::sub_command(),
            functions::closures::closure_examples::sub_command(),
            functions::closures::closure_examples::iter_any::sub_command(),
            functions::closures::closure_examples::iter_find::sub_command(),
            functions::hof::sub_command(),
            functions::diverging::sub_command(),
            // rust-by-example:modules
            modules::sub_command(),
            modules::visibility::sub_command(),
            modules::struct_visibility::sub_command(),
            modules::uses::sub_command(),
            modules::supers::sub_command(),
            modules::split::sub_command(),
            // rust-by-example:crates
            crates::sub_command(),
            crates::lib::sub_command(),
            crates::using_lib::sub_command(),
            // rust-by-example:cargo
            cargo::sub_command(),
            cargo::deps::sub_command(),
            cargo::conventions::sub_command(),
            cargo::test::sub_command(),
            cargo::build_scripts::sub_command(),
            // rust-by-example:attribute
            attribute::sub_command(),
            attribute::unused::sub_command(),
            attribute::cfg::sub_command(),
            attribute::cfg::custom::sub_command(),
            // rust-by-example:generics
            generics::sub_command(),
            generics::gen_fn::sub_command(),
            generics::impls::sub_command(),
            generics::gen_trait::sub_command(),
            generics::bounds::sub_command(),
            generics::bounds::testcase_empty::sub_command(),
            generics::multi_bounds::sub_command(),
            generics::wheres::sub_command(),
            generics::new_types::sub_command(),
            generics::assoc_items::sub_command(),
            generics::assoc_items::the_problem::sub_command(),
            generics::assoc_items::types::sub_command(),
            generics::phantom::sub_command(),
            generics::phantom::testcase_units::sub_command(),
            // rust-by-example:scope
            scope::sub_command(),
            scope::raii::sub_command(),
            scope::moves::sub_command(),
            scope::moves::mutability::sub_command(),
            scope::moves::partial_move::sub_command(),
            scope::borrow::sub_command(),
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
        Some((flow_control::whiles::NAME, sub_matches)) => flow_control::whiles::sub_handler(sub_matches),
        Some((flow_control::fors::NAME, sub_matches)) => flow_control::fors::sub_handler(sub_matches),
        Some((flow_control::matchs::NAME, sub_matches)) => flow_control::matchs::sub_handler(sub_matches),
        Some((flow_control::matchs::destructuring::destructure_tuple::NAME, sub_matches)) => flow_control::matchs::destructuring::destructure_tuple::sub_handler(sub_matches),
        Some((flow_control::matchs::destructuring::destructure_enum::NAME, sub_matches)) => flow_control::matchs::destructuring::destructure_enum::sub_handler(sub_matches),
        Some((flow_control::matchs::destructuring::destructure_pointers::NAME, sub_matches)) => flow_control::matchs::destructuring::destructure_pointers::sub_handler(sub_matches),
        Some((flow_control::matchs::destructuring::destructure_structures::NAME, sub_matches)) => flow_control::matchs::destructuring::destructure_structures::sub_handler(sub_matches),
        Some((flow_control::matchs::guard::NAME, sub_matches)) => flow_control::matchs::guard::sub_handler(sub_matches),
        Some((flow_control::matchs::binding::NAME, sub_matches)) => flow_control::matchs::binding::sub_handler(sub_matches),
        Some((flow_control::if_let::NAME, sub_matches)) => flow_control::if_let::sub_handler(sub_matches),
        Some((flow_control::while_let::NAME, sub_matches)) => flow_control::while_let::sub_handler(sub_matches),
        // rust-by-example:functions
        Some((functions::NAME, sub_matches)) => functions::sub_handler(sub_matches),
        Some((functions::methods::NAME, sub_matches)) => functions::methods::sub_handler(sub_matches),
        Some((functions::closures::NAME, sub_matches)) => functions::closures::sub_handler(sub_matches),
        Some((functions::closures::capture::NAME, sub_matches)) => functions::closures::capture::sub_handler(sub_matches),
        Some((functions::closures::input_parameters::NAME, sub_matches)) => functions::closures::input_parameters::sub_handler(sub_matches),
        Some((functions::closures::anonymity::NAME, sub_matches)) => functions::closures::anonymity::sub_handler(sub_matches),
        Some((functions::closures::input_functions::NAME, sub_matches)) => functions::closures::input_functions::sub_handler(sub_matches),
        Some((functions::closures::output_parameters::NAME, sub_matches)) => functions::closures::output_parameters::sub_handler(sub_matches),
        Some((functions::closures::closure_examples::NAME, sub_matches)) => functions::closures::closure_examples::sub_handler(sub_matches),
        Some((functions::closures::closure_examples::iter_any::NAME, sub_matches)) => functions::closures::closure_examples::iter_any::sub_handler(sub_matches),
        Some((functions::closures::closure_examples::iter_find::NAME, sub_matches)) => functions::closures::closure_examples::iter_find::sub_handler(sub_matches),
        Some((functions::hof::NAME, sub_matches)) => functions::hof::sub_handler(sub_matches),
        Some((functions::diverging::NAME, sub_matches)) => functions::diverging::sub_handler(sub_matches),
        // rust-by-example:modules
        Some((modules::NAME, sub_matches)) => modules::sub_handler(sub_matches),
        Some((modules::visibility::NAME, sub_matches)) => modules::visibility::sub_handler(sub_matches),
        Some((modules::struct_visibility::NAME, sub_matches)) => modules::struct_visibility::sub_handler(sub_matches),
        Some((modules::uses::NAME, sub_matches)) => modules::uses::sub_handler(sub_matches),
        Some((modules::supers::NAME, sub_matches)) => modules::supers::sub_handler(sub_matches),
        Some((modules::split::NAME, sub_matches)) => modules::split::sub_handler(sub_matches),
        Some((crates::NAME, sub_matches)) => crates::sub_handler(sub_matches),
        Some((crates::lib::NAME, sub_matches)) => crates::lib::sub_handler(sub_matches),
        Some((crates::using_lib::NAME, sub_matches)) => crates::using_lib::sub_handler(sub_matches),
        // rust-by-example:cargo
        Some((cargo::NAME, sub_matches)) => cargo::sub_handler(sub_matches),
        Some((cargo::deps::NAME, sub_matches)) => cargo::deps::sub_handler(sub_matches),
        Some((cargo::conventions::NAME, sub_matches)) => cargo::conventions::sub_handler(sub_matches),
        Some((cargo::test::NAME, sub_matches)) => cargo::test::sub_handler(sub_matches),
        Some((cargo::build_scripts::NAME, sub_matches)) => cargo::build_scripts::sub_handler(sub_matches),
        // rust-by-example:attribute
        Some((attribute::NAME, sub_matches)) => attribute::sub_handler(sub_matches),
        Some((attribute::unused::NAME, sub_matches)) => attribute::unused::sub_handler(sub_matches),
        Some((attribute::cfg::NAME, sub_matches)) => attribute::cfg::sub_handler(sub_matches),
        Some((attribute::cfg::custom::NAME, sub_matches)) => attribute::cfg::custom::sub_handler(sub_matches),
        // rust-by-example:generics
        Some((generics::NAME, sub_matches)) => generics::sub_handler(sub_matches),
        Some((generics::gen_fn::NAME, sub_matches)) => generics::gen_fn::sub_handler(sub_matches),
        Some((generics::impls::NAME, sub_matches)) => generics::impls::sub_handler(sub_matches),
        Some((generics::gen_trait::NAME, sub_matches)) => generics::gen_trait::sub_handler(sub_matches),
        Some((generics::bounds::NAME, sub_matches)) => generics::bounds::sub_handler(sub_matches),
        Some((generics::bounds::testcase_empty::NAME, sub_matches)) => generics::bounds::testcase_empty::sub_handler(sub_matches),
        Some((generics::multi_bounds::NAME, sub_matches)) => generics::multi_bounds::sub_handler(sub_matches),
        Some((generics::wheres::NAME, sub_matches)) => generics::wheres::sub_handler(sub_matches),
        Some((generics::new_types::NAME, sub_matches)) => generics::new_types::sub_handler(sub_matches),
        Some((generics::assoc_items::NAME, sub_matches)) => generics::assoc_items::sub_handler(sub_matches),
        Some((generics::assoc_items::the_problem::NAME, sub_matches)) => generics::assoc_items::the_problem::sub_handler(sub_matches),
        Some((generics::assoc_items::types::NAME, sub_matches)) => generics::assoc_items::types::sub_handler(sub_matches),
        Some((generics::phantom::NAME, sub_matches)) => generics::phantom::sub_handler(sub_matches),
        Some((generics::phantom::testcase_units::NAME, sub_matches)) => generics::phantom::testcase_units::sub_handler(sub_matches),
        // rust-by-example:scope
        Some((scope::NAME, sub_matches)) => scope::sub_handler(sub_matches),
        Some((scope::raii::NAME, sub_matches)) => scope::raii::sub_handler(sub_matches),
        Some((scope::moves::NAME, sub_matches)) => scope::moves::sub_handler(sub_matches),
        Some((scope::moves::mutability::NAME, sub_matches)) => scope::moves::mutability::sub_handler(sub_matches),
        Some((scope::moves::partial_move::NAME, sub_matches)) => scope::moves::partial_move::sub_handler(sub_matches),
        Some((scope::borrow::NAME, sub_matches)) => scope::borrow::sub_handler(sub_matches),

        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}