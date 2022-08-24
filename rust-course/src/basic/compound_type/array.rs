use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/compound_type/array";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/array.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 数组三要素：
// 长度固定
// 元素必须有相同的类型
// 依次线性排列


pub fn sub_handler(_matches :&ArgMatches){
    // 类型推导
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("{:?}", months);

    // 显示类型
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", b);

    // 重复值声明语法糖
    let a = [3; 5];

    println!("{:?}", a);

    // 索引访问
    let a = [9, 8, 7, 6, 5];
    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素

    println!("{}", first);
    println!("{}", second);

    // 数组切片
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 数组切片的特点：
    // 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    // 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理

    // ===== 数组综合使用实例 =======
    // 编译器自动推导出one的类型
    let one             = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3]    = [1, 2, 3];
    let blank1          = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }

    // 做个总结，数组虽然很简单，但是其实还是存在几个要注意的点：
    //
    // 数组类型容易跟数组切片混淆，[T;n]描述了一个数组的类型，而[T]描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
    // [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
    // 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小

}