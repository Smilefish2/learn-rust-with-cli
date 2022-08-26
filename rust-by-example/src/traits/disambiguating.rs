use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:trait/disambiguating";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/trait/disambiguating.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
一个类型可以实现许多不同的 trait。如果两个 trait 都需要相同的名称怎么办？例如，许多 trait 可能拥有名为 get() 的方法。
他们甚至可能有不同的返回类型！

有个好消息：由于每个 trait 实现都有自己的 impl 块，因此很清楚您要实现哪个 trait 的 get 方法。

何时需要调用这些方法呢？为了消除它们之间的歧义，我们必须使用完全限定语法（Fully Qualified Syntax）。
 */
trait UsernameWidget {
    // 从这个 widget 中获取选定的用户名
    fn get(&self) -> String;
}

trait AgeWidget {
    // 从这个 widget 中获取选定的年龄
    fn get(&self) -> u8;
}

// 同时具有 UsernameWidget 和 AgeWidget 的表单
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let form = Form{
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果取消注释此行，则会收到一条错误消息，提示 “multiple `get` found”（找到了多个`get`）。
    // 因为毕竟有多个名为 `get` 的方法。
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}