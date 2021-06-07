// Outer Module
pub mod client;

pub mod network;

/* Inner definition
mod client {

    fn connect() {}
}

mod network {
    fn connect() {}

    // Module in Module
    mod server {
        fn connect() {}
    }
}
 */

#[cfg(test)]
mod tests {
    #[test]
    fn super_demo1() {
        use crate::client::connect;
        connect();
    }

    #[test]
    fn super_demo2() {
        use super::client::connect;
        connect();
    }
}
