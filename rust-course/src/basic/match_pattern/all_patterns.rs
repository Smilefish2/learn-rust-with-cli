use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/match-pattern/all-patterns";
const ABOUT: &str = "https://course.rs/match-pattern/all-patterns.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum MessageColor {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[allow(dead_code)]
struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

pub fn sub_handler(_matches :&ArgMatches){

    // 1.匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 2.匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 3.单分支多模式
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 4.通过序列 ..= 匹配值的范围
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 5.解构并分解值

    // 解构结构体
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    // let Point { x, y } = p;// 简易写法，与字段同名
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 匹配任意单个指定值相等或者全部都有值
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    let msg = MessageColor::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        MessageColor::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        MessageColor::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }

    // 解构结构体和元组
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    println!("{}", feet);
    println!("{}", inches);
    println!("{}", x);
    println!("{}", y);

    // 6.忽略模式中的值
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);// 第一个参数传递的值 3

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            setting_value = new_setting_value;// 匹配并修改值
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // 还可以在一个模式中的多处使用下划线来忽略特定值，如下所示，这里忽略了一个五元元组中的第二和第四个值：
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    // 使用下划线开头忽略未使用的变量
    let _x = 5;

    // 注意, 只使用 _ 和使用以下划线开头的名称有些微妙的不同：比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);// _s报错，_则不会，体会不上句的含义

    // 用 .. 忽略剩余值
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
        // 这里列出了 x 值，接着使用了 .. 模式来忽略其它字段，
        // 这样的写法要比一一列出其它字段，然后用 _ 忽略简洁的多。
    }

    // 还可以用 .. 来忽略元组中间的某些值：
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
            // 这里用 first 和 last 来匹配第一个和最后一个值。.. 将匹配并忽略中间的所有值。
        },
    }

    // 然而使用 .. 必须是无歧义的。如果期望匹配和忽略的值是不明确的，Rust 会报错。下面代码展示了一个带有歧义的 .. 例子，因此不能编译：
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    // 7.匹配守卫提供的额外条件
    let num = Some(4);

    match num {
        // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，
        // 它能为分支模式提供更进一步的匹配条件
        // 这个条件可以使用模式中创建的变量：
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 匹配守卫中使用外部变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 注意 y 是外部变量
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // 或 运算符 |  + 匹配守卫
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // 8.@绑定
    enum MessageBind {
        Hello { id: i32 },
    }

    let msg = MessageBind::Hello { id: 5 };

    match msg {
        MessageBind::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        MessageBind::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        MessageBind::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

    // @前绑定后解构(Rust 1.56 新增)
    // 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);


    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    // @新特性(Rust 1.53 新增)
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}