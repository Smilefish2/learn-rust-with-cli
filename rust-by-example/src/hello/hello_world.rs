use clap::{App, ArgMatches};

pub const COMMAND_NAME: &'static str = "rust-by-example:hello_word";
const COMMAND_ABOUT: &'static str = "https://doc.rust-lang.org/rust-by-example/hello.html";

pub fn sub_command<'a, 'b>() -> App<'a, 'b> {
    let sub_command = App::new(COMMAND_NAME)
        .about(COMMAND_ABOUT);
    return sub_command;
}

pub fn sub_handler(matches :&ArgMatches){
    if let Some(_matches) = matches.subcommand_matches(COMMAND_NAME) {


        // Statements here are executed when the compiled binary is called

        // Print text to the console
        println!("Hello World!");

    }
}