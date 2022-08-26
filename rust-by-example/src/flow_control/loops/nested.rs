use clap::{App, ArgMatches};


pub const NAME: &str = "rust-by-example:flow_control/loop/nested";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/flow_control/loop/nested.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
在处理嵌套循环的时候可以 break 或 continue 外层循环。在这类情形中，循环必须 用一些 'label（标签）来注明，并且标签必须传递给 break/continue 语句。
 */
#[allow(unused_labels)]
#[allow(unreachable_code)]
pub fn sub_handler(_matches :&ArgMatches){
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}