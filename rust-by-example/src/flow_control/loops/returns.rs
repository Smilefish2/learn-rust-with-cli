use clap::{App, ArgMatches};


pub const NAME: &str = "rust-by-example:flow_control/loop/return";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/loop/return.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
loop 有个用途是尝试一个操作直到成功为止。若操作返回一个值，则可能需要将其传递 给代码的其余部分：将该值放在 break 之后，它就会被 loop 表达式返回。
 */
pub fn sub_handler(_matches :&ArgMatches){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Exited the outer loop");
}