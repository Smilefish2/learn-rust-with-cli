use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:error/multiple_error_types/option_result";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/option_result.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
处理混合错误类型的最基本的手段就是让它们互相包含。
 */

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

// 有时候我们不想再处理错误（比如使用 ? 的时候），但如果 Option 是 None 则继续处理错误。一些组合算子可以让我们轻松地交换 Result 和 Option。

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

pub fn sub_handler(_matches :&ArgMatches){
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number



    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first2(numbers));
    println!("The first doubled is {:?}", double_first2(empty));
    println!("The first doubled is {:?}", double_first2(strings));
}
