use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:variable_bindings/declare";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/declare.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
可以先声明（declare）变量绑定，后面才将它们初始化（initialize）。但是这种做法很 少用，因为这样可能导致使用未初始化的变量。
 **/

pub fn sub_handler(_matches :&ArgMatches){
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    // println!("another binding: {}", another_binding);
    // 改正 ^ 注释掉此行

    another_binding = 1;

    println!("another binding: {}", another_binding);

    // 编译器禁止使用未经初始化的变量，因为这会产生未定义行为（undefined behavior）。
}

