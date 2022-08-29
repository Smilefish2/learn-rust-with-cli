pub mod vector;
pub mod hashmap;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/collections";
const ABOUT: &str = "https://course.rs/basic/collections/intro.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("https://course.rs/basic/collections/intro.html");
}