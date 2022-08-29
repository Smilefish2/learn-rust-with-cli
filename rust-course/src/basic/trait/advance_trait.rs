use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/trait/advance-trait";
const ABOUT: &str = "https://course.rs/basic/trait/advance-trait.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    // TODO
    // TODO
    // TODO
    // TODO
    // TODO
    // TODO
    // TODO
    println!("https://course.rs/basic/trait/advance-trait.html");
}