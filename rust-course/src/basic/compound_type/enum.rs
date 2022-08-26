use clap::{App, ArgMatches};

pub const NAME: &str = "rust-course:basic/compound_type/enum";
const ABOUT: &str = "https://course.rs/basic/compound-type/enum.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_suit(card: PokerSuit) {
    println!("{:?}",card);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn sub_handler(_matches :&ArgMatches){
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);


    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');

    println!("{:?}", c1);
    println!("{:?}", c2);


    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);
}