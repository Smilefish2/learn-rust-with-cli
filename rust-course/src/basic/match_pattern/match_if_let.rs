use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/match-if-let";
const ABOUT: &str = "https://course.rs/match-pattern/match-if-let.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

#[allow(dead_code)]
enum Direction{
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin)-> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime=> 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[allow(dead_code)]
enum IpAddr {
    Ipv4,
    Ipv6
}

#[allow(dead_code)]
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}
#[derive(Debug)]
#[allow(dead_code)]
enum MyEnum {
    Foo,
    Bar
}

pub fn sub_handler(_matches :&ArgMatches){

    // match 例子
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println ! ("South or North");
        },
        _ => println!("West"),
    }

    // match 匹配
    // match target {
    //     模式1 => 表达式1,
    //     模式2 => {
    //         语句1;
    //         语句2;
    //         表达式2
    //     },
    //     _ => 表达式3
    // }

    let coin = Coin::Quarter(UsState::Alabama);
    let coin_value = value_in_cents(coin);

    println!("{}", coin_value);

    // match 表达式赋值
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255,255,0),
    ];

    for action in actions {
        match action{
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r,g,_) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                         r, g,
                );
            }
        }
    }

    // _ 通配符
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let 匹配
    let v = Some(3u8);
    match v {
        Some(3) => println!("three 1"),
        _ => (),
    }
    if let Some(3) = v {
        println!("three 2");
    }

    // matches!宏
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    // v.iter().filter(|x| matches!(x, MyEnum::Foo));// ??? unused `Filter` that must be used !!!
    println!("{:?}", v);

    // 变量覆盖(match 中的变量覆盖其实不是那么的容易看出, 因此要小心！)
    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    if let Some(age) = age {
        println!("匹配出来的age是{}",age);
    }

    println!("在匹配后，age是{:?}",age);

    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    match age {
        Some(age) =>  println!("匹配出来的age是{}",age),
        _ => ()
    }
    println!("在匹配后，age是{:?}",age);
}