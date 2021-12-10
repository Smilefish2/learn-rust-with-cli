// pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:macros/syntax/repeat";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/macros/syntax/repeat.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 * 来表示该 参数可能出现零次或多次。

在下面例子中，把模式这样： $(...),+ 包围起来，就可以匹配一个或多个用逗号隔开 的表达式。另外注意到，宏定义的最后一个分支可以不用分号作为结束。
 */

// `min!` 将求出任意数量的参数的最小值。
macro_rules! find_min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对 `$x` 后面的 `$y` 们调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

pub fn sub_handler(_matches :&ArgMatches) {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}