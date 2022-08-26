use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:variable_bindings/freeze";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/freeze.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}


/**
当数据被相同的名称不变地绑定时，它还会冻结（freeze）。在不可变绑定超出作用域之前，无法修改已冻结的数据：
 **/

pub fn sub_handler(_matches :&ArgMatches){
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;
}

