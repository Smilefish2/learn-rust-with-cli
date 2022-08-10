use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-practice:variables";
const ABOUT: &'static str = "https://zh.practice.rs/variables.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    // 绑定和可变性
    // 1.🌟 变量只有在初始化后才能被使用
    let x: i32 = 10; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);

    // 2.🌟🌟 可以使用 mut 将变量标记为可变

    let mut x = 1;
    x += 2;

    println!("x = {}", x);

    // 变量作用域
    //3.🌟 作用域是一个变量在程序中能够保持合法的范围
    let x: i32 = 10;
    let y: i32 = 15;
    {
        let x: i32 = 1;
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y);

    //4.🌟🌟
    let x = "hello";
    println!("{}, world", x);

    #[allow(dead_code)]
    fn define_x() {
        let _x = "hello";
    }

    // 变量遮蔽( Shadowing )
    // 5. 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".

    // 6.🌟🌟 删除一行代码以通过编译
    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    // 遮蔽且再次绑定
    let x = x;
    // x += 3;
    println!("{}", x);

    let _y = 4;
    // 遮蔽
    let _y = "I can also be bound to text!";

    // 使用以下方法来修复编译器输出的 warning :
    // 🌟 一种方法
    // 🌟🌟 两种方法
    let _ = 1;

    // 变量解构
    // 8.🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
    let (x, y) = (1, 2);
    // x += 2;

    assert_eq!(x, 1);
    assert_eq!(y, 2);

    // 解构式赋值
    // 9.🌟🌟
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3, 2]);
}