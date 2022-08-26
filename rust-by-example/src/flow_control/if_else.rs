use clap::{App, ArgMatches};

// pub mod comment;
// pub mod print;

pub const NAME: &str = "rust-by-example:flow_control/if_else";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/if_else.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
if-else 分支判断和其他语言类似。与很多语言不同的是，Rust 语言中的布尔判断条件 不必用小括号包住，且每个条件后面都跟着一个代码块。
if-else 条件选择是一个表达 式，并且所有分支都必须返回相同的类型。
 */

pub fn sub_handler(_matches :&ArgMatches){
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 这个表达式返回一个 `i32` 类型。
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // 这个表达式也必须返回一个 `i32` 类型。
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式。
        };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);
}