pub mod boxs;
pub mod vec;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:std";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/std.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
标准库提供了很多自定义类型，在原生类型基础上进行了大量扩充。这是部分自定义 类型：

    可增长的 String（字符串），如: "hello world"
    可增长的向量（vector）: [1, 2, 3]
    选项类型（optional types）: Option<i32>
    错误处理类型（error handling types）: Result<i32, i32>
    堆分配的指针（heap allocated pointers）: Box<i32>
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/std.html");
}