use clap::{App, ArgMatches};

pub mod destructure_tuple;
pub mod destructure_enum;
pub mod destructure_pointers;
pub mod destructure_structures;

pub const NAME: &'static str = "rust-by-example:flow_control/match/destructuring";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring.html";

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
    let pair = (0, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        // 解构出第二个值
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}