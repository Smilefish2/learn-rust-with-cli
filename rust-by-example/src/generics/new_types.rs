use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-by-example:generics/new_types";
const ABOUT: &'static str = "https://rustwiki.org/zh-CN/rust-by-example/generics/new_types.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
new type 惯用法
    newtype 惯用法（译注：即为不同种类的数据分别定义新的类型）能保证在编译时，提供 给程序的都是正确的类型。

    比如说，实现一个 “年龄认证” 函数，它要求输入必须是 Years 类型。
 */

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// 舍去不满一年的部分
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

pub fn sub_handler(_matches :&ArgMatches){
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    // 取消最后一行的注释，就可以发现提供给 old_enough 的必须是 Years 类型。
}