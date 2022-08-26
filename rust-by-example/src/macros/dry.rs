// pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:macros/dry";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/macros/dry.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
通过提取函数或测试集的公共部分，宏可以让你写出 DRY 的代码（DRY 是 Don't Repeat Yourself 的缩写，意思为 “不要写重复代码”）。
这里给出一个例子，对 Vec<T> 实现 并测试了关于 +=、*= 和 -= 等运算符。
 */

pub fn sub_handler(_matches :&ArgMatches) {
    println!("手动执行命令：cd rust-by-example/src/macros && rustc --test dry_test.rs && ./dry_test && cd ../../../");
}