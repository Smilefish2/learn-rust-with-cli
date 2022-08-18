use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/compound_type/tuple";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/tuple.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    // 创建元组
    let tup: (i32, f64, u8) = (500, 6.4 , 1);
    println!("{:?}", tup);
    // 用模式匹配解构元组
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("The value of y is: {}", y);

    // 用 . 来访问元组
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);

    // 元组的使用示例
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);// 注意所有权

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}