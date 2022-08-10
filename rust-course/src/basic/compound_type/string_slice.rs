use clap::{App, ArgMatches};

pub const NAME: &'static str = "rust-course:basic/compound_type/string_slice";
const ABOUT: &'static str = "https://course.rs/basic/compound-type/string-slice.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

pub fn sub_handler(_matches :&ArgMatches){
    let my_name = "Pascal";
    greet(my_name);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello, {}!", hello);
    println!("world, {}!", world);

    let s1 = "中国人";
    let a1 = &s1[0..3];
    println!("{}",a1);

    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);

    let s = "Hello, world!";
    println!("{}", &s[0..2]);


    // 字符串常用操作，大部分只适用String类型，少部分&str类型可用
    let mut s = String::from("Hello ");
    // 追加
    s.push('r');
    println!("追加字符 push() -> {}", s);
    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);

    // 插入
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 替换
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    // 1、replace
    let new_string_replace = string_replace.replace("rust", "RUST");
    // 2、replacen
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replace);
    dbg!(new_string_replacen);

    // 3、replace_range
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除 (Delete)
    let mut string_pop = String::from("rust pop 中文!");
    // 1、 pop —— 删除并返回字符串的最后一个字符
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // 2、 remove —— 删除并返回字符串中指定位置的字符
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // 3、truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // 4、clear —— 清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接 (Catenate)
    // 1、使用 + 或者 += 连接字符串
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // 2、使用 format! 连接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    // 字符串转义

    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    // 1、\ 转义
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
    // 2、\\  不转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 字符
    for c in "中国人".chars() {
        println!("{}", c);
    }

    // 字节
    for b in "中国人".bytes() {
        println!("{}", b);
    }


    // 获取子串
    // 使用库：utf8_slice



}