pub mod open;
pub mod create;
pub mod read_lines;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std_misc/file";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/file.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
File 结构体表示一个被打开的文件（它包裹了一个文件描述符），并赋予了对所表示的 文件的读写能力。

由于在进行文件 I/O（输入/输出）操作时可能出现各种错误，因此 File 的所有方法都 返回 io::Result<T> 类型，它是 Result<T, io::Error> 的别名。

这使得所有 I/O 操作的失败都变成显式的。借助这点，程序员可以看到所有的失败 路径，并被鼓励主动地处理这些情形。
 */

pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/std_misc/file.html");
}