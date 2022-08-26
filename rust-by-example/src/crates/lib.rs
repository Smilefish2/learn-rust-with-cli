use std::{env};
use std::process::Command;
use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:crates/lib";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/crates/lib.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
编译 rary.rs
*/
pub fn sub_handler(_matches :&ArgMatches){

    println!("手动执行命令：cd rust-by-example/src/crates && rustc --crate-type=lib rary.rs && ls && cd ../../../");

    let current_dir = env::current_dir().unwrap();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    let pwd = current_dir.into_os_string().into_string().unwrap();
    let command = format!("cd {}/rust-by-example/src/crates && rustc --crate-type=lib rary.rs && ls", pwd);

    println!(
        "command = {}",
        command
    );

    let output = Command::new(command)
        .output().expect("执行异常，提示");
    println!("{:?}", output);
}