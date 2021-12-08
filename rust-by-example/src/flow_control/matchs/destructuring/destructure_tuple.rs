use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:flow_control/match/destructuring/destructure_tuple";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring/destructure_tuple.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
解构
    match 代码块能以多种方式解构物件。

    解构元组
    解构枚举
    解构指针
    解构结构体
 */
pub fn sub_handler(_matches :&ArgMatches){
    println!("match 代码块能以多种方式解构物件。");
}