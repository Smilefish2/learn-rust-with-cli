use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/collections/vector";
const ABOUT: &str = "https://course.rs/basic/collections/vector.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 动态数组只能存储相同类型的元素，如果你想存储不同类型的元素，可以使用之前讲过的枚举类型或者特征对象。

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}

trait IpAddress {
    fn display(&self);
}

struct V4(String);
impl IpAddress for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddress for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    // 显示声明类型
    let v1: Vec<i32> = Vec::new();
    println!("{:?}", v1);
    // 自动推导类型
    let mut v2 = Vec::new();
    v2.push(1);// 此处推导
    println!("{:?}", v2);
    // vec![]宏创建并初始化值
    let v3 = vec![1, 2, 3];
    println!("{:?}", v3);

    // 下标读取
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    // get 下标
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // 和其它语言一样，集合类型的索引下标都是从 0 开始，
    // &v[2] 表示借用 v 中的第三个元素，最终会获得该元素的引用。而 v.get(2) 也是访问第三个元素，
    // 但是有所不同的是，它返回了 Option<&T>，因此还需要额外的 match 来匹配解构出具体的值。

    // 下标索引与 .get 的区别
    // 确保索引不会越界，使用下标方式访问
    // 不确定索引会不会越界时，使用.get访问

    // 迭代遍历 Vector 中的元素
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
    // 迭代过程中，修改 Vector 中的元素值
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    // 枚举存储不同类型的元素
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }

    // 特征存储不同类型的元素
    let v: Vec<Box<dyn IpAddress>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }

    // 在实际使用场景中，特征对象数组要比枚举数组常见很多，主要原因在于特征对象非常灵活，而编译器对枚举的限制较多，且无法动态增加类型。
}