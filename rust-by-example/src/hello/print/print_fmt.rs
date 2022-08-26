use std::fmt::{self, Formatter, Display};
use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:hello/print/fmt";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/hello/print/fmt.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**

我们已经看到，格式化的方式是通过格式字符串来指定的：

format!("{}", foo) -> "3735928559"
format!("0x{:X}", foo) -> "0xDEADBEEF"
format!("0o{:o}", foo) -> "0o33653337357"
根据使用的参数类型是 X、o 还是未指定，同样的变量（foo）能够格式化 成不同的形式。

这个格式化的功能是通过 trait 实现的，每种参数类型都对应一种 trait。最常见的格式 化 trait 就是 Display，它可以处理参数类型为未指定的情况，比如 {}。

 **/

pub fn sub_handler(_matches :&ArgMatches){

    struct City {
        name: &'static str,
        // 纬度
        lat: f32,
        // 经度
        lon: f32,
    }

    impl Display for City {
        // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
            // 一个缓冲区（即第一个参数f）中。
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]// remove warning: field is never read: `xxx`
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}



