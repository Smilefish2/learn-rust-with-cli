use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std_misc/wait";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/wait.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**

如果你想等待一个 process::Child 完成，就必须调用 Child::wait，这会返回 一个 process::ExitStatus。

 */

use std::process::Command;

pub fn sub_handler(_matches :&ArgMatches){
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}