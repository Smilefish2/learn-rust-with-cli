use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:ownership:ownership";
const ABOUT: &'static str = "https://course.rs/basic/ownership/ownership.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    // 动态字符串，同时声明为可修改mut
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('1');
    println!("{}", s);

    // 所有权转移：称之为移动（move）
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // 实际上， String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针是最重要的，
    // 它指向了真实存储字符串内容的堆内存，至于长度和容量，如果你有 Go 语言的经验，这里就很好理解：容量是堆内存分配空间的大小，长度是目前已经使用的大小。
    //
    // 总之 String 类型指向了一个堆上的空间，这里存储着它的真实数据, 下面对上面代码中的 let s2 = s1 分成两种情况讨论：
    //
    // 拷贝 String 和存储在堆上的字节数组 如果该语句是拷贝所有数据(深拷贝)，那么无论是 String 本身还是底层的堆上数据，都会被全部拷贝，这对于性能而言会造成非常大的影响
    //
    // 只拷贝 String 本身 这样的拷贝非常快，因为在 64 位机器上就拷贝了 8字节的指针、8字节的长度、8字节的容量，总计 24 字节，
    // 但是带来了新的问题，还记得我们之前提到的所有权规则吧？其中有一条就是：一个值只允许有一个所有者，而现在这个值（堆上的真实字符串数据）有了两个所有者：s1 和 s2。
    //
    // 好吧，就假定一个值可以拥有两个所有者，会发生什么呢？
    //
    // 当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。不过由于两个 String 变量指向了同一位置。
    // 这就有了一个问题：当 s1 和 s2 离开作用域，它们都会尝试释放相同的内存。这是一个叫做 二次释放（double free） 的错误，
    // 也是之前提到过的内存安全性 BUG 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
    //
    // 因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，
    // 这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    //
    // 再来看看，在所有权转移后再来使用旧的所有者，会发生什么：

    // println!("{}", s1);
    //              ^^ value borrowed here after move

    // !!!
    // Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);

    // 克隆(深拷贝)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝(浅拷贝)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    // 函数传值与返回
    // 将值传递给函数，一样会发生 移动 或者 复制，就跟 let 语句一样，下面的代码展示了所有权、作用域的规则：
    //
    //
    // fn main() {
    //     let s = String::from("hello");  // s 进入作用域
    //
    //     takes_ownership(s);             // s 的值移动到函数里 ...
    //     // ... 所以到这里不再有效
    //
    //     let x = 5;                      // x 进入作用域
    //
    //     makes_copy(x);                  // x 应该移动函数里，
    //     // 但 i32 是 Copy 的，所以在后面可继续使用 x
    //
    // } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // // 所以不会有特殊操作
    //
    // fn takes_ownership(some_string: String) { // some_string 进入作用域
    //     println!("{}", some_string);
    // } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
    //
    // fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    //     println!("{}", some_integer);
    // } // 这里，some_integer 移出作用域。不会有特殊操作
    // 你可以尝试在 takes_ownership 之后，再使用 s，看看如何报错？例如添加一行 println!("在move进函数后继续使用s: {}",s);。
    //
    // 同样的，函数返回值也有所有权，例如:
    //
    //
    // fn main() {
    //     let s1 = gives_ownership();         // gives_ownership 将返回值
    //                                         // 移给 s1
    //
    //     let s2 = String::from("hello");     // s2 进入作用域
    //
    //     let s3 = takes_and_gives_back(s2);  // s2 被移动到
    //                                         // takes_and_gives_back 中,
    //                                         // 它也将返回值移给 s3
    // } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    //   // 所以什么也不会发生。s1 移出作用域并被丢弃
    //
    // fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    //     // 调用它的函数
    //
    //     let some_string = String::from("hello"); // some_string 进入作用域.
    //
    //     some_string                              // 返回 some_string 并移出给调用的函数
    // }
    //
    // // takes_and_gives_back 将传入字符串并返回该值
    // fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    //     a_string  // 返回 a_string 并移出给调用的函数
    // }
    // 所有权很强大，避免了内存的不安全性，但是也带来了一个新麻烦： 总是把一个值传来传去来使用它。
    // 传入一个函数，很可能还要从该函数传出去，结果就是语言表达变得非常啰嗦，幸运的是，Rust 提供了新功能解决这个问题。
}