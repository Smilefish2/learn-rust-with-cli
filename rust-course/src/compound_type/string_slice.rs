use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:compound_type/string_slice";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/string-slice.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


fn greet(name: &str) {
    println!("Hello, {}!", name);
}

pub fn sub_handler(_matches :&ArgMatches){
    let my_name = "Pascal";
    greet(my_name);
}