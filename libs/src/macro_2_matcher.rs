#[macro_export]
macro_rules! yo {
    ($name:expr) => {
        println!("Yo, {}!", $name);
    };
}

macro_rules! multi_yo {
    ($($name:expr), *) => {
        $(println!("Yo, {}!", $name);)*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hey_macro() {
        yo! {"jasonkay"}
        yo!("jasonkay");

        multi_yo!("jasonkay1", "jasonkay2", "jasonkay3");
    }
}
