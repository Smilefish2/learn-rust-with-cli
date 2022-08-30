pub mod bubble_sort;

use clap::{App, ArgMatches};

pub const NAME: &str = "rust-algos:sorting";
const ABOUT: &str = "https://algos.rs/sorting/index.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    println!("参见：https://algos.rs/sorting/index.html");

}