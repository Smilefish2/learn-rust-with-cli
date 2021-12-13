use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:error/multiple_error_types/boxing_errors";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/boxing_errors.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们装箱（Box）。
这 样做的坏处就是，被包装的错误类型只能在运行时了解，而不能被静态地 判别。

对任何实现了 Error trait 的类型，标准库的 Box 通过 From 为它们提供了 到 Box<Error> 的转换。
 */

use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
