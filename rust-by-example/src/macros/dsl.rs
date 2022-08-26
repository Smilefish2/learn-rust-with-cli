// pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:macros/dsl";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/macros/dsl.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
DSL 是 Rust 的宏中集成的微型 “语言”。这种语言是完全合法的，因为宏系统会把它转换 成普通的 Rust 语法树，它只不过看起来像是另一种语言而已。这就允许你为一些特定功能 创造一套简洁直观的语法（当然是有限制的）。

比如说我想要定义一套小的计算器 API，可以传给它表达式，它会把结果打印到控制台上。
 */
macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // 强制类型为整型
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

pub fn sub_handler(_matches :&ArgMatches) {
    calculate! {
        eval 1 + 2 // 看到了吧，`eval` 可并不是 Rust 的关键字！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}