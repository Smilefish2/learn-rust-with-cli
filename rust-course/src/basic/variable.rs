use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/variable";
const ABOUT: &str = "https://course.rs/basic/variable.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

struct Struct {
    e: i32
}

const MAX_POINTS: u32 = 100_000;// 下划线等于,千位符，提高可读性

pub fn sub_handler(_matches :&ArgMatches){
    // 变量绑定 && 变量可变性
    let mut  x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);

    // 使用下划线开头忽略未使用的变量
    let _y = 10;
    let _ = 11;// 完全忽略

    // 使用 let 变量解构
    let (a, mut b) : (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 解构式赋值( rust version > 1.59 )
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 变量和常量之间的差异
    println!("a = {:?}", MAX_POINTS);


    // 变量遮蔽(shadowing)

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}