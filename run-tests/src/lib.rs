pub fn print_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use crate::{print_and_return_10, add_two};

    #[test]
    fn pass_test() {
        let val = print_and_return_10(4);
        assert_eq!(val, 10);
    }

    #[test]
    fn fail_test() {
        let val = print_and_return_10(8);
        assert_eq!(val, 5);
    }

    #[test]
    fn add_for_2() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_for_3() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn add_for_100() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        println!("test will be ignored");
    }

}
