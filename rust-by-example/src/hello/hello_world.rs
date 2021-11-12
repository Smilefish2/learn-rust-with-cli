use clap::{App, Arg, ArgMatches};

pub fn sub_command<'a, 'b>() -> App<'a, 'b> {
    let sub_command = App::new("test")
        .about("does testing things")
        .arg(
            Arg::with_name("list")
                .short("l")
                .help("lists test values")
        );
    return sub_command;
}

pub fn sub_handler(matches :&ArgMatches){
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}