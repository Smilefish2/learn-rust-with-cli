use clap::{App, ArgMatches};
use std::collections::HashMap;

pub const NAME: &str = "rust-course:basic/collections/hashmap";
const ABOUT: &str = "https://course.rs/basic/collections/hashmap.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){
    // 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = HashMap::new();
    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    println!("{:?}", my_gems);

    // 使用迭代器和 collect 方法创建
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:?}",teams_map);

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

    println!("{:?}",teams_map);

    // 所有权转移
    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);

    // 查询 HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get获取单值
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    match score {
        Some(&a)=> println!("score:{}", a),
        None => println!("score:None"),
    }

    // 遍历 KV
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新 HashMap 中的值
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入



    // 在已有值的基础上更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


    // 高性能三方库
    use std::hash::BuildHasherDefault;
    // 引入第三方的哈希函数
    use twox_hash::XxHash64;

    // 指定HashMap使用第三方的哈希函数XxHash64
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}