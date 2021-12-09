use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:cargo/build_scripts";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/cargo/build_scripts.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**

*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/cargo/build_scripts.html")
}