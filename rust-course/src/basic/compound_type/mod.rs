pub mod string_slice;
pub mod tuple;
pub mod structs;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/compound_type";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}", "https://course.rs/basic/compound-type/intro.html");
}