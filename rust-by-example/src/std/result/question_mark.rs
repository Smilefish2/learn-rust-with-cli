use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:std/result/question_mark";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/std/result/question_mark.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
把 result 用 match 连接起来会显得很难看；幸运的是，? 运算符可以把这种逻辑变得 干净漂亮。
? 运算符用在返回值为 Result 的表达式后面，它等同于这样一个匹配 表达式：其中 Err(err) 分支展开成提前返回的 return Err(err)，而 Ok(ok) 分支展开成 ok 表达式。
*/

mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败” 了，那么返回 `DivisionByZero`
        let ratio = div(x, y)?;

        // 如果 `ln` “失败” 了，那么返回 `NonPositiveLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                => "logarithm of non-positive number",
                MathError::DivisionByZero
                => "division by zero",
                MathError::NegativeSquareRoot
                => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    checked::op(1.0, 10.0);
}