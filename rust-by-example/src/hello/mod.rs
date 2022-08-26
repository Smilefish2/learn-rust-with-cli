use clap::{App, ArgMatches};

pub mod comment;
pub mod print;

pub const NAME: &str = "rust-by-example:hello";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/hello.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    // 调用编译生成的可执行文件时，这里的语句将被运行。

    // 将文本打印到控制台
    println!("Hello World!");
}