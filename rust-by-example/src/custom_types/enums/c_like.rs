use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:custom_types/enum/c_like";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum/c_like.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
enum 也可以像 C 语言风格的枚举类型那样使用。
 **/

// 该属性用于隐藏对未使用代码的警告。
#[allow(dead_code)]
// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn sub_handler(_matches :&ArgMatches){

    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

