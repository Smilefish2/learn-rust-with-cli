use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:flow_control/while";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/while.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
while 关键字可以用作当型循环（当条件满足时循环）。

让我们用 while 循环写一下臭名昭著的 FizzBuzz 程序。
 */
pub fn sub_handler(_matches :&ArgMatches){
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}