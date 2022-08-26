use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:flow_control/match/guard";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/guard.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
可以加上 match 卫语句（guard） 来过滤分支。
 */

pub fn sub_handler(_matches :&ArgMatches){
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}