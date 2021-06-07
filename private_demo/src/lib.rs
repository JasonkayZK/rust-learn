mod outermost {
    pub fn middle_fn() {}

    fn middle_secret_fn() {}

    mod inside {
        pub fn inner_fn() {}

        fn inner_secret_fn() {}
    }
}

fn try_me() {
    outermost::middle_fn();
}

#[cfg(test)]
mod tests {}
