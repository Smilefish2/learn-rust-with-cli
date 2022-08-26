use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:cargo/conventions";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/cargo/conventions.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**

*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/cargo/conventions.html")
}