use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:scope/lifetime/methods";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/methods.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
方法的标注和函数类似：
*/

struct Owner(i32);

impl Owner {
    // 标注生命周期，就像独立的函数一样。
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}


pub fn sub_handler(_matches :&ArgMatches){
    let mut owner  = Owner(18);

    owner.add_one();
    owner.print();

    // 译注：方法一般是不需要标明生命周期的，因为 self 的生命周期会赋给所有的输出 生命周期参数，详见 TRPL。
}