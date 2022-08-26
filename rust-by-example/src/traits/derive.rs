use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:trait/derive";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/trait/derive.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。如果 需要更复杂的行为，这些 trait 也可以手动实现。

下面是可以自动派生的 trait：

    比较 trait: Eq, PartialEq, Ord, PartialOrd
    Clone, 用来从 &T 创建副本 T。
    Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
    Hash，从 &T 计算哈希值（hash）。
    Default, 创建数据类型的一个空实例。
    Debug，使用 {:?} formatter 来格式化一个值。
*/


// `Centimeters`，可以比较的元组结构体
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`，可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`，不带附加属性的元组结构体
struct Seconds(i32);

pub fn sub_handler(_matches :&ArgMatches){
    let _one_second = Seconds(1);

    // 报错：`Seconds` 不能打印；它没有实现 `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // 试一试 ^ 取消此行注释

    // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // 试一试 ^ 取消此行注释

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}