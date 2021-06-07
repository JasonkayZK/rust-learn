// Define
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// Define with different type
// A little bit like union in C
#[derive(Debug)]
enum IpAddrWithDiffType {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    init_enum();
}

// Init
fn init_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let home = IpAddrWithDiffType::V4(127, 0, 0, 1);
    let loopback = IpAddrWithDiffType::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}
