// pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:macros/syntax";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/macros/syntax.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
在下面的小节中，我们将展示如何在 Rust 中定义宏。基本的概念有三个：

    模式与指示符
    重载
    重复
 */


pub fn sub_handler(_matches :&ArgMatches) {
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/macros/syntax.html")
}