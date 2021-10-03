use std::error::Error;

pub fn say_hello(name: &str) -> Result<bool, Box<dyn Error>> {
    println!("hello, {}", name);
    Ok(true)
}
