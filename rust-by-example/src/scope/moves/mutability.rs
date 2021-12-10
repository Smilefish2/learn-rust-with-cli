use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:scope/move/mut";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/scope/move/mut.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
当所有权转移时，数据的可变性可能发生改变。
 */
pub fn sub_handler(_matches :&ArgMatches){
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}