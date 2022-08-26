use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:mod/split";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/mod/split.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。
mod my;

fn function() {
    println!("called `function()`");
}

/**
模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中：
 */
pub fn sub_handler(_matches :&ArgMatches) {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}