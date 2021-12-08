pub mod from_into;
pub mod try_from_try_into;
pub mod string;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:conversion";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/conversion.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 使用 trait 解决类型之间的转换问题。
    最一般的转换会用到 From 和 into 两个 trait。
    不过，即便常见的情况也可能会用到特别的 trait，尤其是 从 String 转换到别的类型，以及把别的类型转换到 String 时。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("{}", "Rust 使用 trait 解决类型之间的转换问题。");
    println!("  {}", "最一般的转换会用到 From 和 into 两个 trait。");
    println!("  {}", "不过，即便常见的情况也可能会用到特别的 trait，尤其是 从 String 转换到别的类型，以及把别的类型转换到 String 时。");
}