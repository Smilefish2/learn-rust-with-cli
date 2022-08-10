use std::fmt::Debug;
use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/base-type/function";
const ABOUT: &'static str = "https://course.rs/basic/base-type/function.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 蛇形命名法(snake case)~~~
fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x:i32) -> i32 {
    x + 5
}

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}



pub fn sub_handler(_matches :&ArgMatches){
    println!("{}", add(1,2));

    another_function(5, 6.1);

    let x = plus_five(5);

    println!("The value of x is: {}", x);

    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);

    // Rust 中的特殊返回类型
    // 无返回值()

    // 例如单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：
    // 函数没有返回值，那么返回一个 ()
    // 通过 ; 结尾的表达式返回一个 ()


    // 例如下面的 report 函数会隐式返回一个 ()：
    #[allow(dead_code)]
    fn report<T: Debug>(item: T) {
        println!("{:?}", item);
    }
    // 与上面的函数返回值相同，但是下面的函数显式的返回了 ()：
    #[allow(dead_code)]
    fn clear(text: &mut String) -> () {
        *text = String::from("");
    }

    // 在实际编程中，你会经常在错误提示中看到该 () 的身影出没，假如你的函数需要返回一个 u32 值，但是如果你不幸的以 表达式; 的方式作为函数的最后一行代码，就会报错：
    // fn add(x:u32,y:u32) -> u32 {
    //     x + y;// here
    // }

    // 永不返回的发散函数 !
    // 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
    #[allow(dead_code)]
    fn dead_end() -> ! {
        panic!("你已经到了穷途末路，崩溃吧！");
    }

    fn forever() -> ! {
        loop {
            //...
        };
    }

    forever();
}