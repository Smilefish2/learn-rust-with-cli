use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std/arc";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/std/arc.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
When shared ownership between threads is needed, Arc(Atomic Reference Counted) can be used. This struct, via the Clone implementation can create a reference pointer for the location of a value in the memory heap while increasing the reference counter. As it shares ownership between threads, when the last reference pointer to a value is out of scope, the variable is dropped.

当线程之间需要共享所有权时，可以使用Arc（原子引用计数）。此结构通过克隆实现可以为内存堆中的值位置创建引用指针，同时增加引用计数器。由于它在线程之间共享所有权，当指向某个值的最后一个引用指针超出范围时，将删除该变量
*/
use std::sync::Arc;
use std::thread;


pub fn sub_handler(_matches :&ArgMatches){
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
}

