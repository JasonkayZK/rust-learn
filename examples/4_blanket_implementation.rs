extern crate generic_trait_lifetime;

use std::fmt::Display;
use generic_trait_lifetime::Tweet;

struct Pair<T> {
    x: T,
    y: T,
}

// All Pair type implemented!
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// Only `Display + PartialOrd` type implemented!
impl<T> Pair<T> where T: Display + PartialOrd {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest num is x={}", self.x);
        } else {
            println!("largest num is y={}", self.y);
        }
    }
}

fn main() {
    let a = Pair::new(10, 2);
    a.cmp_display();
    let b = Pair::new("12", "32");
    b.cmp_display();

    let _c = Pair::new(Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you know"),
        reply: false,
        retweet: false,
    }, Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you know"),
        reply: false,
        retweet: false,
    });

    // Error: c has no cmp_display method!
    // c.cmp_display();
}