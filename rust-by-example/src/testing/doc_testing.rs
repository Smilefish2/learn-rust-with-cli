// pub mod threads;

use clap::{App, ArgMatches};
use colored;
use colored::Colorize;

pub const NAME: &'static str = "rust-by-example:testing/doc_testing";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/testing/doc_testing.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
为 Rust 工程编写文档的主要方式是在源代码中写注释。
文档注释使用 markdown 语法 书写，支持代码块。Rust 很注重正确性，这些注释中的代码块也会被编译并且用作测试。
 */

pub fn sub_handler(_matches :&ArgMatches){
    println!("运行：{}", "cargo test rust-by-example/src/testing/doc_testing/test".red());
}