use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:error/panic";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/error/panic.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
我们将要看到的最简单的错误处理机制就是 panic。它会打印一个错误消息，开始 回退（unwind）任务，且通常会退出程序。这里我们显式地在错误条件下调用 panic：
*/

fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

pub fn sub_handler(_matches :&ArgMatches){
    give_princess("teddy bear");
    give_princess("snake");
}