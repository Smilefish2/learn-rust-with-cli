use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/method";
const ABOUT: &str = "https://course.rs/basic/method.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用
// Rust 使用 impl 来定义方法
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle{
        Circle{
            x,
            y,
            radius
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        println!("Called")
    }
}

pub fn sub_handler(_matches :&ArgMatches){

    // 方法的定义和调用
    let circle = Circle::new(1.00,2.0,3.0);

    println!("{}", circle.x);
    println!("{}", circle.y);
    println!("{}", circle.radius);
    println!("{}", circle.area());

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // self、&self 和 &mut self

    // 方法名跟结构体字段名相同
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    // 当我们使用 rect1.width() 时，Rust 知道我们调用的是它的方法，如果使用 rect1.width，则是访问它的字段。


    // 带有多个参数的方法

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 关联函数
    // 这种定义在 impl 中且没有 self 的函数被称之为关联函数： 因为它没有 self，
    // 不能用 f.read() 的形式调用，因此它是一个函数而不是方法，它又在impl 中，与结构体紧密关联，因此称为关联函数。
    // impl Rectangle {
    //     fn new(w: u32, h: u32) -> Rectangle {
    //         Rectangle { width: w, height: h }
    //     }
    // }
    // Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称，出于设计上的考虑，Rust 特地没有用 new 作为关键字
    // :: 语法用于关联函数和模块创建的命名空间的访问

    // 多个 impl 定义
    // Rust 允许我们为一个结构体定义多个 impl 块，目的是提供更多的灵活性和代码组织性，
    // 例如当方法多了后，可以把相关的方法组织在同一个 impl 块中，那么就可以形成多个 impl 块，各自完成一块儿目标：
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    // }
    //
    // impl Rectangle {
    //     fn can_hold(&self, other: &Rectangle) -> bool {
    //         self.width > other.width && self.height > other.height
    //     }
    // }


    // 为枚举实现方法
    let m = Message::Write(String::from("hello"));
    m.call();
}