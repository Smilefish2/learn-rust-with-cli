pub mod raii;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:scope";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/scope.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
作用域在所有权（ownership）、借用（borrow）和生命周期（lifetime）中起着重要 作用。
也就是说，作用域告诉编译器什么时候借用是合法的、什么时候资源可以释放、以及 变量何时被创建或销毁。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("作用域在所有权（ownership）、借用（borrow）和生命周期（lifetime）中起着重要 作用。");
    println!("也就是说，作用域告诉编译器什么时候借用是合法的、什么时候资源可以释放、以及 变量何时被创建或销毁。");
}