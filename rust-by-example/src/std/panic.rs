use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std/panic";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std/panic.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
panic! 宏可用于产生一个 panic （恐慌），并开始回退（unwind）它的栈。
在回退栈 的同时，运行时将会释放该线程所拥有的所有资源，这是通过调用线程中所有对象的 析构函数完成的。

因为我们正在处理的程序只有一个线程，panic! 将会引发程序报告 panic 消息并退出。
*/

// 整型除法（/）的重新实现
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以 0 会引发 panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 此操作将会引发一个任务失败
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` 应当会在此处被销毁
}

// 可以看到，panic! 不会泄露内存：
// $ rustc panic.rs && valgrind ./panic