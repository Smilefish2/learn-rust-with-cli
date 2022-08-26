use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/trait/generic";
const ABOUT: &'static str = "https://course.rs/basic/trait/generic.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 无范型版本
// 我们在编程中，经常有这样的需求：
// 用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，
// 甚至是自定义类型，都能进行支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数：
fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}

// 范型多态版概念示例（必须使用 std::ops::Add Trait 限制 T 的类型范围，否则不能编译通过，因为不是所有数据类型都可以实现 + 操作）
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
    a + b
}

// 或者使用 + Clone，或者修改为返回 &T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 结构体中使用泛型
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// 方法中使用泛型
impl <T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

// 结构体中使用多个泛型（禁止过多的范型参数个数！！）
#[derive(Debug)]
#[allow(dead_code)]
struct Point2<T,U> {
    x: T,
    y: U,
}

// 除了结构体中的泛型参数，我们还能在该结构体的方法中定义额外的泛型参数，就跟泛型函数一样：
struct Point3<T, U> {
    x: T,
    y: U,
}

// 这个例子中，T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，它们并不冲突，
// 说白了，你可以理解为，一个是结构体泛型，一个是函数泛型。
impl<T, U> Point3<T, U> {
    fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
fn display_array<T: std::fmt::Debug>(arr: &[T]){
    println!("{:?}", arr);
}

// const 泛型
#[allow(dead_code)]
fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]){
    println!("{:?}", arr);
}

// const 泛型表达式
pub enum Assert<const  CHECK: bool>{}
pub trait IsTrue{}
impl IsTrue for Assert<true> {}

// fn something<T: std::fmt::Debug>(val: T) where Assert<{ core::mem::size_of::<T>() < 768 }> : IsTrue,{
//     println!("{}", val);
// }

pub fn sub_handler(_matches :&ArgMatches){
    // 无范型
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));

    // 范型示例
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 结构体单泛型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);

    // 结构体多泛型
    let p = Point2{x: 1, y :1.1};
    println!("{:?}", p);


    // 枚举中使用泛型（Rust内置使用率最高的两个泛型，卧龙凤雏～～）
    // 如果是 Option 是卧龙，那么 Result 就一定是凤雏，得两者可得天下：

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    //
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 方法中使用泛型
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 为具体的泛型类型实现方法
    // 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，
    // 而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。
    // 这个方法计算点实例与坐标(0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 只有T类型为f32类型的Point实例时，distance_from_origin方法可用！
    let p4 = Point { x: 1.0_f32, y: 2.0_f32};
    println!("{}", p4.distance_from_origin());


    // const 泛型（Rust 1.51 版本引入的重要特性）
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr:[i32; 2] = [1, 2];
    display_array(&arr);

    // ??? nightly 版可用 ？？？
    // something([0u8; 0]); // ok
    // something([0u8; 512]); // ok
    // something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制


    // 在 Rust 中泛型是零成本的抽象，意味着你在使用泛型时，完全不用担心性能上的问题。
    //
    // 但是任何选择都是权衡得失的，既然我们获得了性能上的巨大优势，那么又失去了什么呢？
    // Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小。
    //
    // 具体来说：
    // Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
    // 编译器所做的工作正好与我们创建泛型函数的步骤相反，编译器寻找所有泛型代码被调用的位置并针对具体类型生成代码。
    // 让我们看看一个使用标准库中 Option 枚举的例子：
    // let integer = Some(5);
    // let float = Some(5.0);
    // 当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：
    // 一种对应 i32 另一种对应 f64。为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，
    // 接着将泛型定义替换为这两个具体的定义。
    //
    // 编译器生成的单态化版本的代码看起来像这样：
    //
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    //
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    //
    // fn main() {
    //     let integer = Option_i32::Some(5);
    //     let float = Option_f64::Some(5.0);
    // }
    // 我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。
    // 这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。
    // 这个单态化过程正是 Rust 泛型在运行时极其高效的原因。
}