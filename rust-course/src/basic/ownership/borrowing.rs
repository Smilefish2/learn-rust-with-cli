use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/ownership/borrowing";
const ABOUT: &str = "https://course.rs/basic/ownership/borrowing.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string:  &mut String) {
    some_string.push_str(", world");
}

pub fn sub_handler(_matches :&ArgMatches){

    // & 引用
    // * 解引用
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);


    // 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 可变引用同时只能存在一个
    // 可变引用与不可变引用不能同时存在

    // 借用规则总结
    // 总的来说，借用规则如下：
    //
    // 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    // 引用必须总是有效的
}