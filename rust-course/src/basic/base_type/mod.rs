pub mod numbers;
pub mod char_bool;
pub mod statement_expression;
pub mod function;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic:base_type";
const ABOUT: &'static str = "https://course.rs/basic/base-type/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    println!("参见：https://course.rs/basic/base-type/index.html");

    // let _guess = "42".parse().expect("Not a number!");
    let _guess = "42".parse::<i32>().expect("Not a number!");
}