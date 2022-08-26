use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std_misc/pipe";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/pipe.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
std::Child 结构体代表了一个正在运行的子进程，它暴露了 stdin（标准 输入），stdout（标准输出） 和 stderr（标准错误） 句柄，从而可以通过管道与 所代表的进程交互。
 */

use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &str =
    "the quick brown fox jumped over the lazy dog\n";

pub fn sub_handler(_matches :&ArgMatches){
    // 启动 `wc` 命令
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // 将字符串写入 `wc` 的 `stdin`。
    //
    // `stdin` 拥有 `Option<ChildStdin>` 类型，不过我们已经知道这个实例不为空值，
    // 因而可以直接 `unwrap 它。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // 因为 `stdin` 在上面调用后就不再存活，所以它被 `drop` 了，管道也被关闭。
    //
    // 这点非常重要，因为否则 `wc` 就不会开始处理我们刚刚发送的输入。

    // `stdout` 字段也拥有 `Option<ChildStdout>` 类型，所以必需解包。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}