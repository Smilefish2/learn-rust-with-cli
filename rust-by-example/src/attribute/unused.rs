// pub mod deps;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:attribute/unused";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/attribute/unused.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
编译器提供了 dead_code（死代码，无效代码）lint，这会对未使用的函数 产生警告。可以用一个属性来禁用这个 lint。
注意在实际程序中，需要将死代码清除掉。由于本书的例子是交互性的，因而其中需要 允许一些死代码的出现。
 */

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]

pub fn sub_handler(_matches :&ArgMatches){
    fn used_function() {}

    fn unused_function() {}

    fn noisy_unused_function() {}
    // 改正 ^ 增加一个属性来消除警告

    fn main() {
        used_function();
    }
}