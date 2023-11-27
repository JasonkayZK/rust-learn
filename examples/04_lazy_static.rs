use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Jack, Allen"));
}

fn main() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}
