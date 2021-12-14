pub mod threads;
pub mod channels;
pub mod path;
pub mod file;
pub mod process;
pub mod fs;
pub mod arg;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:std_misc";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
标准库也提供了很多其他类型来支持某些功能，例如：

    线程（Threads）
    信道（Channels）
    文件输入输出（File I/O）

这些内容在原生类型之外进行了有效扩充。
 */
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/std_misc.html");
}