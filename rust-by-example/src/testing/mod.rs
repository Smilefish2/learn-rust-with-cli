pub mod unit_testing;
pub mod doc_testing;
pub mod integration_testing;

use clap::{App, ArgMatches};
use colored::*;

pub const NAME: &'static str = "rust-by-example:testing";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/testing.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 是一门非常重视正确性的语言，这门语言本身就提供了对编写软件测试的支持。

测试有三种风格：
    单元测试。
    文档测试。
    集成测试。
Rust 也支持在测试中指定额外的依赖：
    开发依赖
 */
pub fn sub_handler(_matches :&ArgMatches){
    println!("{}","参见：https://rustwiki.org/zh-CN/rust-by-example/testing.html".red());
}