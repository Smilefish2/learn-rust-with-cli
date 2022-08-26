use clap::{App, ArgMatches};
use num::complex::Complex;

pub const NAME: &str = "rust-course:basic/base-type/numbers";
const ABOUT: &str = "https://course.rs/basic/base-type/numbers.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 整数类型^_^
// 长度	有符号类型	无符号类型
// 8 位	    i8	    u8
// 16 位	i16	    u16
// 32 位	i32	    u32
// 64 位	i64	    u64
// 128 位	i128	u128
// 视架构而定	isize	usize

// 整形字面量可以用下表的形式书写：
// 数字字面量	    示例
// 十进制	    98_222
// 十六进制	    0xff
// 八进制	    0o77
// 二进制	    0b1111_0000
// 字节 (仅限于 u8)	b'A'

// 整型溢出!!!
// 在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。
// 相反，当检测到整型溢出时，Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理

pub fn sub_handler(_matches :&ArgMatches){
    // 浮点类型^_^
    // 断言0.1 + 0.2与0.3相等
    // assert!(0.1 + 0.2 == 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    // 数字运算^_^
    // 加法
    let sum = 5 + 10;
    println!("+:{}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("-:{}", difference);

    // 乘法
    let product = 4 * 30;
    println!("*:{}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("/:{}", quotient);

    // 求余
    let remainder = 43 % 5;
    println!("%:{}", remainder);

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    // 位运算
    // 运算符	说明
    // & 位与	相同位置均为1时则为1，否则为0
    // | 位或	相同位置只要有1时则为1，否则为0
    // ^ 异或	相同位置不相同则为1，相同则为0
    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    // << 左移	所有位向左移动指定位数，右位补零
    // >> 右移	所有位向右移动指定位数，左位补零

    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);

    // 序列(Range)^_^
    for i in 1..=5 {
        println!("{}",i);
    }

    for i in 'a'..='z' {
        println!("{}",i);
    }

    // 有理数和复数^_^
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}