pub mod cast;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:types";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/types.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 提供了多种机制，用于改变或定义原生类型和用户定义类型。接下来会讲到：
    原生类型的类型转换（cast）。
    指定字面量的类型。
    使用类型推断（type inference）。
    给类型取别名（alias）。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("{}", "Rust 提供了多种机制，用于改变或定义原生类型和用户定义类型。接下来会讲到：");
    println!("  {}", "原生类型的类型转换（cast）。");
    println!("  {}", "使用类型推断（type inference）。");
    println!("  {}", "给类型取别名（alias）。");
}