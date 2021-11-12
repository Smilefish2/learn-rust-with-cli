use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:hello/print";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/hello/print.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**

打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：

format!：将格式化文本写到字符串（String）。（译注：字符串是返 回值不是参数。）
print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
println!: 与 print! 类似，但输出结果追加一个换行符。
eprint!：与 format! 类似，但将文本输出到标准错误（io::stderr）。
eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
这些宏都以相同的做法解析（parse）文本。另外有个优点是格式化的正确性会在编译时检查。

**/

pub fn sub_handler(_matches :&ArgMatches){
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

//
// std::fmt 包含多种 traits（trait 有 “特征，特性” 等意思） 来控制文字显示，其中重要的两种 trait 的基本形式如下：
//
// fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
// fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。
// 上例使用了 fmt::Display，因为标准库提供了那些类型的实现。若要打印自定义类型的 文本，需要更多的步骤。

