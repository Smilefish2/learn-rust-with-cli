// pub mod cast;
use std::string::ToString;
use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:conversion/string";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/conversion/string.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
ToString

    要把任何类型转换成 String，只需要实现那个类型的 ToString trait。然而不要直接这么做，您应该实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型，就像 print! 一节中讨论的那样。
*/

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}