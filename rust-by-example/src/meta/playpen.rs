use clap::{App, ArgMatches};
use colored::*;

pub const NAME: &str = "rust-by-example:meta/playpen";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/meta/playpen.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust Playpen 是一个在线运行 Rust 代码的网络接口。现在该项目通常称为 Rust Playground。
 */
pub fn sub_handler(_matches :&ArgMatches) {
    println!("参见：{}","https://rustwiki.org/zh-CN/rust-by-example/meta/playpen.html".red());
}