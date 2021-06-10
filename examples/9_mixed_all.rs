use std::fmt::{Display, Debug};

fn main() {
    println!("longest_with_an_announcement: {}", longest_with_an_announcement("longest", "short", "hello"));
    println!("longest_with_an_announcement: {}", longest_with_an_announcement("longest", "short", 123));
    println!("longest_with_an_announcement: {}", longest_with_an_announcement("longest", "short", 123.123));
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Debug + Display {
    println!("announcement: {:?}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
