#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello: {}", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rectangle, add_two, greeting, Guess};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Panic demo: Fail test
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Macro assert! demo
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    // Macro assert_eq! & assert_ne! demo
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    // Macro assert! with format
    #[test]
    fn greeting_contains() {
        let res = greeting("Jasonkay");
        assert!(
            res.contains("ZK"),
            "Greeting did not contain name, value was: `{}`", res
        );
    }

    // Should panic demo
    #[test]
    #[should_panic]
    fn greater_than() {
        Guess::new(101);
    }

    // Should panic & expected demo
    #[test]
    #[should_panic(expected = "value must be between 1 and 100")] // sub-str to panic!
    fn greater_than2() {
        Guess::new(101);
    }
}

