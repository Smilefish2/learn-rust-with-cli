use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/trait/trait-object";
const ABOUT: &'static str = "https://course.rs/basic/trait/trait-object.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// Rust 没有继承，特征对象就类似继承

#[allow(dead_code)]
pub trait Draw{
    fn draw(&self);
}
#[allow(dead_code)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
        println!("Button Draw");
    }
}
#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("SelectBox Draw");
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches){

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();


    // 特征对象的动态分发 dyn 关键字

    // Self 与 self
    // 在 Rust 中，有两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名：
    //
    // trait Draw {
    //     fn draw(&self) -> Self;
    // }
    //
    // #[derive(Clone)]
    // struct Button;
    // impl Draw for Button {
    //     fn draw(&self) -> Self {
    //         return self.clone()
    //     }
    // }
    //
    // fn main() {
    //     let button = Button;
    //     let newb = button.draw();
    // }
    // 上述代码中，self指代的就是当前的实例对象，也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。
    //
    // 当理解了 self 与 Self 的区别后，我们再来看看何为对象安全。


    // 特征对象的限制
    // 不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    // 1.方法的返回类型不能是 Self
    // 2.方法没有任何泛型参数

}