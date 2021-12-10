// pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:macros/syntax/overload";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/macros/syntax/overload.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
宏的参数使用一个美元符号 $ 作为前缀，并使用一个指示符（designator）来 注明类型：
 */
// 根据你调用它的方式，`test!` 将以不同的方式来比较 `$left` 和 `$right`。
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

pub fn sub_handler(_matches :&ArgMatches) {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}