use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic:base_type:statement_expression";
const ABOUT: &'static str = "https://course.rs/basic/base-type/statement-expression.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值，例如：
#[allow(dead_code)]
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式!!!
}

pub fn sub_handler(_matches :&ArgMatches){
    // 语句
    let a = 8;
    println!("{}", a);
    let b: Vec<f64> = Vec::new();
    println!("{:?}", b);
    let (a, c) = ("hi", false);
    println!("{}", a);
    println!("{}", c);

    // 表达式，切记表达式不可以用;号结束，否则会隐式返回单元类型()
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    fn ret_unit_type() {
        let x = 1;
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        if x > 1 {

        }
    }

    assert_eq!(ret_unit_type(), ())
}