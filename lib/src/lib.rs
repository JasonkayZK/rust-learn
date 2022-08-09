pub mod dynamic;

#[cfg(test)]
mod tests {
    use crate::dynamic;

    #[test]
    fn test_say_hello() {
        dynamic::do_stuff();
    }
}
