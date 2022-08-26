use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:fn/closures/closure_examples/iter_find";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/fn/closures/closure_examples/iter_find.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
Iterator::find 是一个函数，在传给它一个迭代器时，将用 Option 类型返回第一个 满足谓词的元素。它的签名如下：

    pub trait Iterator {
        // 被迭代的类型。
        type Item;

        // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
        // 但不会被消耗。
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
            // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
            // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型）
            P: FnMut(&Self::Item) -> bool {}
    }
*/

pub fn sub_handler(_matches :&ArgMatches){
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}

