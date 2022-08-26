pub mod generics;
pub mod r#trait;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/trait";
const ABOUT: &'static str = "https://course.rs/basic/trait.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    println!("https://course.rs/basic/trait/intro.html");
}