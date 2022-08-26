// pub mod visibility;

use clap::{App, ArgMatches};
use colored::*;

pub const NAME: &str = "rust-by-example:meta/doc";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/meta/doc.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// #[crate_name = "doc"]
/// 这里给出一个人类
pub struct Person {
    /// 一个人必须有名字，不管 Juliet 多讨厌他/她。
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人物的名称
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中，你可以书写代码块
    /// // 如果向 Rustdoc 传递 --test 参数，它还会帮你测试注释文档中的代码！
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(& self) {
        println!("Hello, {}!", self.name.red());
    }
}
pub fn sub_handler(_matches :&ArgMatches) {
    let john = Person::new("John");

    john.hello();

    // 要运行测试，首先将代码构建为库，然后告诉 rustdoc 在哪里找到库，这样它就可以 使每个文档中的程序链接到库：
    // $ rustc doc.rs --crate-type lib
    // $ rustdoc --test --extern doc="libdoc.rlib" doc.rs
    // （当你对库 crate 上运行 cargo test 时，Cargo 将自动生成并运行正确的 rustc 和 rustdoc 命令。）
}