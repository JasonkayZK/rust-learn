#[derive(Debug)]
struct User {
    // Invalid: lack of lifetime for reference!
    // username: &str,
    // email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {}