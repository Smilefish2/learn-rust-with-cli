pub mod panics;
pub mod option_unwrap;
pub mod result;
pub mod multiple_error_types;
pub mod iter_result;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:error";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/error.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
错误处理（error handling）是处理可能发生的失败情况的过程。
例如读取一个文件时 失败了，如果继续使用这个无效的输入，那显然是有问题的。
注意到并且显式地 处理这种错误可以避免程序的其他部分产生潜在的问题。

在 Rust 中有多种处理错误的方式，在接下来的小节中会一一介绍。
它们多少有些 区别，使用场景也不尽相同。总的来说：

显式的 panic 主要用于测试，以及处理不可恢复的错误。
在原型开发中这很有用，比如 用来测试还没有实现的函数，不过这时使用 unimplemented 更能表达意图。
另外在 测试中，panic 是一种显式地失败（fail）的好方法。
Option 类型是为了值是可选的、或者缺少值并不是错误的情况准备的。
比如说寻找 父目录时，/ 和 C: 这样的目录就没有父目录，这应当并不是一个错误。
当处理 Option 时，unwrap 可用于原型开发，也可以用于能够确定 Option 中一定有值 的情形。
然而 expect 更有用，因为它允许你指定一条错误信息，以免万一还是出现 了错误。
当错误有可能发生，且应当由调用者处理时，使用 Result。
你也可以 unwrap 然后 使用 expect，但是除了在测试或者原型开发中，请不要这样做。
有关错误处理的更多内容，可参考官方文档的错误处理的章节。
*/
pub fn sub_handler(_matches :&ArgMatches){
    println!("参见：https://rustwiki.org/zh-CN/rust-by-example/error.html");
}