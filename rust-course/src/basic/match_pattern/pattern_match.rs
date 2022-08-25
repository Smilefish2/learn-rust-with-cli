use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/match-pattern/pattern_match";
const ABOUT: &'static str = "https://course.rs/match-pattern/pattern_match.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    // 1.所有可能用到模式的地方
    // match 分支, 匹配所有
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    // 2.特殊模式 _ ,匹配剩余的所有情况
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     _ => EXPRESSION,
    // }

    // 3.if let 分支
    // if let PATTERN = SOME_VALUE {
    //
    // }

    // 4.while let 条件循环
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //  5.for 循环
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 6.let 语句
    // let PATTERN = EXPRESSION;
    // let x = 5;
    // 这其中，x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中,变量名也是一种模式，只不过它比较朴素很不起眼罢了。
    // let (x, y, z) = (1, 2, 3);
    // 上面将一个元组与模式进行匹配(模式和值的类型必需相同！)，然后把 1, 2, 3 分别绑定到 x, y, z 上。
    // 模式匹配要求两边的类型必须相同，否则就会报错：
    // let (x, y) = (1, 2, 3);
    //     ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
    // 对于元组来说，元素个数也是类型的一部分！！！

    // 7.函数参数
    // 函数参数也是模式：
    // fn foo(x: i32) {
    //     // 代码
    // }
    // 其中 x 就是一个模式，你还可以在参数中匹配元组：
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

    // 8.if 和 if let
    // if 和 if let
    // 对于以下代码，编译器会报错：
    // let Some(x) = some_option_value;
    // 因为右边的值可能不为 Some，而是 None，这种时候就不能进行匹配，也就是上面的代码遗漏了 None 的匹配。
    //
    // 类似 let 和 for、match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )。
    //
    // 但是对于 if let，就可以这样使用：
    //
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    // 因为 if let 允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )。
}