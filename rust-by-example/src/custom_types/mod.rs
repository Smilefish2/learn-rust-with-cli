pub mod structs;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:custom_types";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/custom_types.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 自定义数据类型主要是通过下面这两个关键字来创建：

struct： 定义一个结构体（structure）
enum： 定义一个枚举类型（enumeration）
而常量（constant）可以通过 const 和 static 关键字来创建。
*/
pub fn sub_handler(_matches :&ArgMatches){

}