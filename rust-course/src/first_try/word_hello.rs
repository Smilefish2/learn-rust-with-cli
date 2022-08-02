use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:first-try/word_hello";
const ABOUT: &'static str = "https://course.rs/first-try/hello-world.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    greet_world();
}