use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:generics/multi_bounds";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/generics/multi_bounds.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
多重约束

多重约束（multiple bounds）可以用 + 连接。和平常一样，类型之间使用 , 隔开。

 */

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

pub fn sub_handler(_matches :&ArgMatches){
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。

    compare_types(&array, &vec);
}