// Define
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    init_enum();

    route(IpAddrKind::V4);
}

// Init
fn init_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:#?}", four);
    println!("six: {:#?}", six);
}

// Enum in func
fn route(ip_type: IpAddrKind) {
    println!("six: {:#?}", ip_type);
}


