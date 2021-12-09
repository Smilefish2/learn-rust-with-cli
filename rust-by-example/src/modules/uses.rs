use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:mod/use";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/mod/use.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
use 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问。
 */
// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches) {
    // 更容易访问 `deeply::nested::funcion`
    other_function();

    println!("Entering block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将遮蔽外部的同名函数。
        use deeply::nested::function;
        function();

        // `use` 绑定拥有局部作用域。在这个例子中，`function()`
        // 的遮蔽只存在在这个代码块中。
        println!("Leaving block");
    }

    function();
}