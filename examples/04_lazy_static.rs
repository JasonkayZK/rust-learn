use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Jack, Allen"));
}

fn main() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}",v);
}
