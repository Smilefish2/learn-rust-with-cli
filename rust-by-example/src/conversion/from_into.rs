// pub mod cast;
use std::convert::From;
use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:conversion/from_into";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
From 和 Into
    From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能把类型 B 转换为类型 A。
From
    From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。

    比如，可以很容易地把 str 转换成 String：

    #![allow(unused)]
    fn main() {
        let my_str = "hello";
        let my_string = String::from(my_str);
    }
也可以为我们自己的类型定义转换机制：

 */



#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
pub fn sub_handler(_matches :&ArgMatches){
    let num = Number::from(30);
    println!("My number is {:?}", num);
}