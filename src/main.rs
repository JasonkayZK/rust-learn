pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    println!("Hello, world on build version: {}", VERSION);
}
