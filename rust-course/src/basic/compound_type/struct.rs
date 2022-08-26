use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/compound_type/struct";
const ABOUT: &str = "https://course.rs/basic/compound-type/struct.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 定义一个解构体
#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

struct AlwaysEqual;

trait SomeTrait{
    fn test(&self);
}

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {
    fn test(&self){
        println!("AlwaysEqual");
    }
}

// 如果你想在结构体中使用一个引用，就必须加上生命周期
#[derive(Debug)]
struct Lifetimes<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn sub_handler(_matches :&ArgMatches){

    // 有几点值得注意:
    //     初始化实例时，每个字段都需要进行初始化
    //     初始化时的字段顺序不需要和结构体定义时的顺序一致
    let user1 = User{
        email:String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    println!("{:?}", user1);

    // 需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变。
    // user1.username = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("email@abc.com"), String::from("username"));

    println!("{:?}", user2);

    // 结构体更新语法
    let user3 = User{
      email: String::from("another@example.com"),
        ..user1 // 必须在结构体的尾部使用!
    };

    // 仔细回想一下所有权那一节的内容，我们提到了 Copy 特征：实现了 Copy 特征的类型无需所有权转移，
    // 可以直接在赋值时进行 数据拷贝，其中 bool 和 u64 类型就实现了 Copy 特征，
    // 因此 active 和 sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，而不是所有权转移。
    // 值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，
    // 但是并不代表 user1 内部的其它字段不能被继续使用：


    println!("{:?}", user3);

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    // 元组结构体(Tuple Struct)
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);

    //单元结构体(Unit-like Struct)
    let subject = AlwaysEqual;
    subject.test();

    // 结构体数据的所有权
    let life1 = Lifetimes {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    println!("{}", life1.email);
    println!("{}", life1.username);
    println!("{}", life1.active);
    println!("{}", life1.sign_in_count);
    println!("{:#?}", life1);

    // dbg! 宏
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("{}", rect1.width);
    println!("{}", rect1.height);
    dbg!(&rect1);
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}