pub mod pipe;

use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:std_misc/process";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/process.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
process::Output 结构体表示已结束的子进程（child process）的输出，而 process::Command 结构体是一个进程创建者（process builder）。
 */
use std::process::Command;

pub fn sub_handler(_matches :&ArgMatches){
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}