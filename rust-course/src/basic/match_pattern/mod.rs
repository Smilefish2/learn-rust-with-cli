pub mod match_if_let;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/match-pattern";
const ABOUT: &'static str = "https://course.rs/match-pattern/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("https://course.rs/basic/match-pattern/intro.html");
}