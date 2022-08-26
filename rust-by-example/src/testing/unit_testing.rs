// pub mod threads;

use clap::{App, ArgMatches};
use colored;
use colored::Colorize;

pub const NAME: &str = "rust-by-example:testing/unit_testing";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/testing/unit_testing.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
测试（test）是这样一种 Rust 函数：它保证其他部分的代码按照所希望的行为正常 运行。
测试函数的函数体通常会进行一些配置，运行我们想要测试的代码，然后 断言（assert）结果是不是我们所期望的。

大多数单元测试都会被放到一个叫 tests 的、带有 #[cfg(test)] 属性 的模块中，测试函数要加上 #[test] 属性。

当测试函数中有什么东西 panic 了，测试就失败。有一些这方面的 辅助宏：

    assert!(expression) - 如果表达式的值是 false 则 panic。
    assert_eq!(left, right) 和 assert_ne!(left, right) - 检验左右两边是否 相等/不等。

运行特定的测试
    要运行特定的测试，只要把测试名称传给 cargo test 命令就可以了。
$ cargo test test_any_panic
running 1 test
test tests::test_any_panic ... ok

要运行多个测试，可以仅指定测试名称中的一部分，用它来匹配所有要运行的测试。

$ cargo test panic
running 2 tests
test tests::test_any_panic ... ok
test tests::test_specific_panic ... ok
 */

pub fn sub_handler(_matches :&ArgMatches){
    println!("运行：{}", "cargo test rust-by-example/src/testing/unit_testing/test".red());
    println!("运行：{}", "cargo test rust-by-example/src/testing/unit_testing/panic".red());
    println!("运行：{}", "cargo test rust-by-example/src/testing/unit_testing/ignore".red());
}