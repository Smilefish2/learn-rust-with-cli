use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/match-pattern/option";
const ABOUT: &'static str = "https://course.rs/match-pattern/option.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}