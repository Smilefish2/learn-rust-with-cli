use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:error/multiple_error_types/reenter_question_mark";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/reenter_question_mark.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
注意在上一个例子中，我们调用 parse 后总是立即把错误从标准库错误 map 到装箱的错误。

.and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())

因为这个操作很简单常见，如果有省略写法就好了。and_then 不够灵活，所以不能实现 这样的写法。不过，我们可以使用 ? 来代替它。

之前我们说 ? 就是 “要么 unwrap 要么 return Err(error)”，这大部分是对的，但 事实上 ? 是 “要么 unwrap 要么 return Err(From::from(err))”。
From::from 是 不同类型间的转换工具，也就是说，如果在错误能够转换成返回类型的地方使用 ?，它就 会自动转换成返回类型。

这里，我们使用 ? 重写之前的例子。这样，只要为我们的错误类型实现 From::from，就 可以不再使用 map_err。
 */

use std::error;
use std::fmt;

// Change the alias to `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

// 这段代码现在已经很清晰了。相比原始的 panic，它就像是把所有的 unwrap 调用都换 成 ? 一样。
// 与 panic 相比，这样做的区别在于返回类型是 Result，因而必须在顶层 解构它们。
