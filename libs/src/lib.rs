pub mod hello;

#[cfg(test)]
mod tests {
    use crate::hello::say_hello;

    #[test]
    fn test_say_hello() {
        match say_hello("rust") {
            Ok(res) => {
                assert_eq!(res, true);
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
