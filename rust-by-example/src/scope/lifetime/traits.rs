use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:scope/lifetime/trait";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/trait.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
trait 方法中生命期的标注基本上与函数类似。注意，impl 也可能有生命周期的标注。
*/

// 带有生命周期标注的结构体。
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// 给 impl 标注生命周期。
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}