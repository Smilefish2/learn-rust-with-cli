pub mod raw_identifiers;

use clap::{App, ArgMatches};
use colored::*;

pub const NAME: &'static str = "rust-by-example:compatibility";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/compatibility.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
兼容性
    Rust 语言正在快速发展，因此尽管努力确保尽可能向前兼容，但仍可能出现某些兼容性问题。
*/

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}","https://rustwiki.org/zh-CN/rust-by-example/compatibility.html".red());
}