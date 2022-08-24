use clap::{App, ArgMatches};
// use chrono::Local;

pub const NAME: &'static str = "rust-course:basic/flow-control";
const ABOUT: &'static str = "https://course.rs/basic/flow-control.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    let condition = true;

    if condition == true {
        println!("true");
    } else {
        println!("false");
    }
    // 使用if语句表达返回值
    let number = if condition == true { 5 } else { 6 };
    println!("The value of number is:{}", number);

    // else if
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // for循环
    for i in -5..=5  {
        println!("{}", i);
    }
    // 使用方法	                    等价使用方式	                                        所有权
    // for item in collection	    for item in IntoIterator::into_iter(collection)	    转移所有权
    // for item in &collection	    for item in collection.iter()	                    不可变借用
    // for item in &mut collection	for item in collection.iter_mut()	                可变借用


    // 获取索引
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // for循环中使用忽略符号
    for _ in 1..10 {
        println!("_");
    }

    // 两种循环方式优劣对比
    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("第一种：{}", item);
    }

    // 第二种(严重推荐！！！)
    for item in collection {
        println!("第二种：{}", item);
    }
    // 优劣如下：
    //
    // 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，
    //     但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    // 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，
    //     导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险（这里是因为所有权吗？是的话可能要强调一下）


    // continue， 跳过当前当次的循环，开始下次的循环
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("continue：{}", i);
    }

    // break 直接跳出当前整个循环：
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("break：{}", i);
    }

    // while 循环
    let mut n = 0;

    while n <= 5 {
        println!("while: {}", n);
        n = n + 1;
    }

    println!("我出来了！");

    // loop 循环

    let mut n = 0;

    loop {
        if n > 5 {
            break
        }
        println!("{}", n);

        n += 1;
    }

    // while 实现 for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    // for避免运行时的边界检查，性能更高
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 无限循环
    // let local_time = Local::now();
    // loop {
    //     println!("again! {:?}" ,local_time);
    // }


    // loop + break
    // break 可以单独使用，也可以带一个返回值，有些类似 return
    // loop 是一个表达式，因此可以返回一个值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}