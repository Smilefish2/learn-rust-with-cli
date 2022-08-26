use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:std/rc";
const ABOUT: &str = "https://doc.rust-lang.org/rust-by-example/std/rc.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
When multiple ownership is needed, Rc(Reference Counting) can be used. Rc keeps track of the number of the references which means the number of owners of the value wrapped inside an Rc.

Reference count of an Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is dropped out of the scope. When an Rc's reference count becomes zero, which means there are no owners remained, both the Rc and the value are all dropped.

Cloning an Rc never performs a deep copy. Cloning creates just another pointer to the wrapped value, and increments the count.


当需要多重所有权时，可以使用Rc（引用计数）。Rc跟踪引用的数量，这意味着包装在Rc中的值的所有者数量。

每当一个Rc被克隆时，Rc的引用计数增加1，而每当一个克隆的Rc从作用域中删除时，Rc的引用计数减少1。当一个Rc的引用计数变为零时，这意味着没有剩余的所有者，Rc和该值都将被删除。

克隆Rc永远不会执行深度复制。克隆只创建另一个指向包装值的指针，并增加计数。
*/

use std::rc::Rc;

pub fn sub_handler(_matches :&ArgMatches){
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
}