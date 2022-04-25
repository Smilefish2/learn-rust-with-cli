use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:compound_type/string_slice";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/string-slice.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

pub fn sub_handler(_matches :&ArgMatches){
    let my_name = "Pascal";
    greet(my_name);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello, {}!", hello);
    println!("world, {}!", world);

    let s1 = "中国人";
    let a1 = &s1[0..3];
    println!("{}",a1);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}