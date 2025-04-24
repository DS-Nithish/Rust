#![allow(unused)]

use std::rc::Rc;

use chrono::prelude::*;

enum Greeting {
    Morning,
    AfterNoon,
    Evening,
    Night,
}

fn event() {
    let time = Local::now();
    let ff = Timelike::hour(&time);
    //println!("{time:?}");
    //println!("{ff:?}");

    match ff {
        6..=11 => println!("Good Morning"),
        12..=16 => println!("Good AfterNoon"),
        17..=19 => println!("Good Evening"),
        20..=23 => println!("Good Night"),
        _ => println!("nothing"),
    }
}

fn main() {
    event();
}
