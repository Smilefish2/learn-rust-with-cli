use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:attribute/cfg/custom";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/attribute/cfg/custom.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
有部分条件如 target_os 是由 rustc 隐式地提供的，但是自定义条件必须使用 --cfg 标记来传给 rustc。
*/

// #[cfg(some_condition)] //!!!! here
fn conditional_function() {
    println!("condition met!")
}

pub fn sub_handler(_matches :&ArgMatches){
    conditional_function();

    /*
    试试不使用自定义的 cfg 标记会发生什么：

    $ rustc custom.rs && ./custom
    No such file or directory (os error 2)
    使用自定义的 cfg 标记：


    $ rustc --cfg some_condition custom.rs && ./custom
    condition met!
    */
}