use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:expression";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/expression.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 程序（大部分）由一系列语句构成：

fn main() {
    // 语句
    // 语句
    // 语句
}
Rust 有多种语句。最普遍的语句类型有两种：一种是绑定变量，另一种是表达式带上分号：

fn main() {
    // 变量绑定
    let x = 5;

    // 表达式;
    x;
    x + 1;
    15;
}
代码块也是表达式，所以它们在赋值操作中可以充当右值（r-values）。
代码块中的最后一条 表达式将赋给左值（l-value）。
需要注意的是，如果代码块最后一条表达式结尾处有分号，那 么返回值将变成 ()。
（译注：代码块中的最后一条语句是代码块中实际执行的最后一条语句，而不一 定是代码块中最后一行的语句。）
*/
#[allow(warnings)]
pub fn sub_handler(_matches :&ArgMatches){
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}