use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:std_misc/threads";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/std_misc/threads.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Rust 通过 spawn 函数提供了创建本地操作系统（native OS）线程的机制，该函数的参数是一个通过值捕获变量的闭包（moving closure）。
 */

use std::thread;

static NTHREADS: i32 = 10;

pub fn sub_handler(_matches :&ArgMatches){
    // 提供一个 vector 来存放所创建的子线程（children）。
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 启动（spin up）另一个线程
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // 等待线程结束。返回一个结果。
        let _ = child.join();
    }
}