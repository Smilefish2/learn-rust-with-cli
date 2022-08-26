pub mod matching;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std_misc/arg";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/arg.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
标准库
    命令行参数可使用 std::env::args 进行接收，这将返回一个迭代器，该迭代器会对 每个参数举出一个字符串。
 */

use std::env;

pub fn sub_handler(_matches :&ArgMatches){
    let args: Vec<String> = env::args().collect();

    // 第一个参数是调用本程序的路径
    println!("My path is {}.", args[0]);

    // 其余的参数是被传递给程序的命令行参数。
    // 请这样调用程序：
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    //
    // $ ./args 1 2 3
    // My path is ./args.
    //     I got 3 arguments: ["1", "2", "3"].

    // crate
    // 另外，也有很多 crate 提供了编写命令行应用的额外功能。Rust Cookbook 展示了使用 最流行的命令行参数 crate，即 clap 的最佳实践。
}