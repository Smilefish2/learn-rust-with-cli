pub mod doc;
pub mod playpen;

use clap::{App, ArgMatches};
use colored::*;

pub const NAME: &'static str = "rust-by-example:meta";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/meta.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
在软件工程中，有些主题和写程序并没有直接的关联，但它们为你提供了工具和基础设施 支持，使得软件对每个人都变得更易用。这些主题包括：

文档：通过附带的 rustdoc 生成库文档给用户。
测试：为库创建测试套件，确保库准确地实现了你想要的功能。
基准测试（benchmark）：对功能进行基准测试，保证其运行速度足够快。
 */
pub fn sub_handler(_matches :&ArgMatches) {
    println!("参见：{}","https://rustwiki.org/zh-CN/rust-by-example/meta.html".red());
}