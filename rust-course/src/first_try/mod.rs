pub mod word_hello;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:first-try";
const ABOUT: &'static str = "https://course.rs/first-try/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}", "https://course.rs/first-try/intro.html");
}