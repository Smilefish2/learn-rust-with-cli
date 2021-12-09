use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:fn/closures/input_functions";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/fn/closures/input_functions.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
输入函数
    既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个 接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数。
*/

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}
pub fn sub_handler(_matches :&ArgMatches){
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    // 多说一句，Fn、FnMut 和 FnOnce 这些 trait 明确了闭包如何从周围的作用域 中捕获变量。
}

