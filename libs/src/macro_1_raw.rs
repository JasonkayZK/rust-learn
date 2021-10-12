#[macro_export]
macro_rules! hey {
    () => {};
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hey_macro() {
        hey! {}
        hey!();
    }
}
