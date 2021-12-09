pub mod iter_any;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:fn/closures/closure_examples";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/fn/closures/closure_examples.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
方法（method）是依附于对象的函数。这些方法通过关键字 self 来访问对象中的数据和 其他。方法在 impl 代码块中定义。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("本小节列出几个标准库中使用闭包的例子。");
}