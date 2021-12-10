pub mod unused;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:attribute";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/attribute.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
属性
属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：
    条件编译代码
    设置 crate 名称、版本和类型（二进制文件或库）
    禁用 lint （警告）
    启用编译器的特性（宏、全局导入（glob import）等）
    链接到一个非 Rust 语言的库
    标记函数作为单元测试
    标记函数作为基准测试的某个部分
当属性作用于整个 crate 时，它们的语法为 #![crate_attribute]，当它们用于模块 或项时，语法为 #[item_attribute]（注意少了感叹号 !）。

属性可以接受参数，有不同的语法形式：
    #[attribute = "value"]
    #[attribute(key = "value")]
    #[attribute(value)]
属性可以多个值，它们可以分开到多行中：


#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}", "https://rustwiki.org/zh-CN/rust-by-example/attribute.html");
}