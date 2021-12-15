// pub mod threads;

use clap::{App, ArgMatches};
use colored;
use colored::Colorize;

pub const NAME: &'static str = "rust-by-example:testing/dev_dependencies";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/testing/dev_dependencies.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
开发依赖
有时仅在测试中才需要一些依赖（比如基准测试相关的）。这种依赖要写在 Cargo.toml 的 [dev-dependencies] 部分。这些依赖不会传播给其他依赖于这个包的包。

比如说使用 pretty_assertions，这是扩展了标准的 assert! 宏的一个 crate。

文件 Cargo.toml:


# 这里省略了标准的 crate 数据
[dev-dependencies]
pretty_assertions = "0.4.0"


文件 src/lib.rs:

// 仅用于测试的外部 crate
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
 */

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：{}","https://rustwiki.org/zh-CN/rust-by-example/testing.html".red());
}