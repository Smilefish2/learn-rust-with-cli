// pub mod cast;
use std::convert::TryFrom;
use std::convert::TryInto;
use clap::{App, ArgMatches};

pub const NAME: &str = "rust-by-example:conversion/try_from_try_into";
const ABOUT: &str = "https://rustwiki.org/zh-CN/rust-by-example/conversion/try_from_try_into.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

/**
类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
 */

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn sub_handler(_matches :&ArgMatches){
// TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}