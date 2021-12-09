pub mod visibility;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:mod";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/mod.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 提供了一套强大的模块（module）系统，可以将代码按层次分成多个逻辑 单元（模块），并管理这些模块之间的可见性（公有（public）或私有（private））

模块是项（item）的集合，项可以是：函数，结构体，trait，impl 块，甚至其它模块
 */
pub fn sub_handler(_matches :&ArgMatches) {
    println!("Rust 提供了一套强大的模块（module）系统，可以将代码按层次分成多个逻辑 单元（模块），并管理这些模块之间的可见性（公有（public）或私有（private））");
    println!("模块是项（item）的集合，项可以是：函数，结构体，trait，impl 块，甚至其它模块");
}