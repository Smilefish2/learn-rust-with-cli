use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:error/result/result_alias";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/error/result/result_alias.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
当我们要重用某个 Result 类型时，该怎么办呢？回忆一下，Rust 允许我们 创建别名。
若某个 Result 有可能被重用，我们可以方便地给它取一个别名。

在模块的层面上创建别名特别有帮助。同一模块中的错误常常会有相同的 Err 类 型，所以单个别名就能简便地定义所有相关的 Result。
这太有用了，以至于标准库 也提供了一个别名： io::Result！

下面给出一个简短的示例来展示语法：
 */

use std::num::ParseIntError;

// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用上面定义过的别名来表示上一节中的 `Result<i32,ParseIntError>` 类型。
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 在这里使用别名又让我们节省了一些代码量。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


pub fn sub_handler(_matches :&ArgMatches){
    print(multiply("10", "2"));
    print(multiply("t", "2"));

}