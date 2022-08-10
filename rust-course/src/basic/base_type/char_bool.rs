use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/base-type/char-bool";
const ABOUT: &'static str = "https://course.rs/basic/base-type/char-bool.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    // 字符类型(char)
    // 单引号界定，双引号为字符串
    // 不仅仅是ASCII,所有的 Unicode 值都可以作为 Rust 字符
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", g);
    println!("{}", heart_eyed_cat);

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    // 布尔(bool)
    let _t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }

    // 单元类型
    // 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
}