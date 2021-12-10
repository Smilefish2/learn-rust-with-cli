use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:scope/lifetime/struct";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/struct.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
在结构体中标注生命周期也和函数的类似：
 */

// 一个 `Borrowed` 类型，含有一个指向 `i32` 类型的引用。
// 该引用必须比 `Borrowed` 寿命更长。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 和前面类似，这里的两个引用都必须比这个结构体长寿。
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// 一个枚举类型，其取值不是 `i32` 类型就是一个指向 `i32` 的引用。
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn sub_handler(_matches :&ArgMatches){
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}