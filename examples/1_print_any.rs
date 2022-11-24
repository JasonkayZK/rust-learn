use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
struct MyType {
    name: String,
    age: u32,
}

fn print_any<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    if let Some(string) = value_any.downcast_ref::<String>() {
        println!("String ({}): {}", string.len(), string);
    } else if let Some(MyType { name, age }) = value_any.downcast_ref::<MyType>() {
        println!("MyType ({}, {})", name, age)
    } else {
        println!("{:?}", value)
    }
}

fn main() {
    let ty = MyType {
        name: "Rust".to_string(),
        age: 30,
    };
    let name = String::from("Rust");

    print_any(&ty);
    print_any(&name);
    print_any(&30);
}
