use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/trait/trait";
const ABOUT: &'static str = "https://course.rs/basic/trait/trait.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

// 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为,特征很类似接口

// 定义特征
pub trait Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
    fn summarize_author(&self) -> String;
}
// 实现特征
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 函数返回中的 impl Trait
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}


// 自定义类型的打印输出
#[allow(dead_code)]
#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>",
               self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches){

    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());

    notify(&post);
    notify(&weibo);

    // 特征约束（单条件约束）
    // 啰嗦（适合不同类型）
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    // 正确（适合同类型多参数）
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {}

    // 多重约束（+号， 两种写法均可）
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}

    // Where 约束
    // 不使用where
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // 使用where
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //     where T: Display + Clone,
    //           U: Clone + Debug
    // {}

    // 使用特征约束有条件地实现方法或特征
    let t= Pair::new(1,2);
    t.cmp_display();

    // 也可返回一个特征
    let summary = returns_summarizable();
    println!("{}",summary.summarize());

    // 通过 derive 派生特征: 如 #[derive(Debug)]
    // 调用方法需要引入特征：如 use std::convert::TryInto;（已在std::prelude,不需要在次引入，可直接使用，不在prelude时需要引用）

    // 几个综合例子
    // 1。为自定义类型实现 + 操作
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));

    // 2.自定义类型的打印输出
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
}