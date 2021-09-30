use std::ops::{Deref, DerefMut};

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

fn mut_hello(name: &mut str) {
    println!("hello, {}", name);
}

fn deref_demo_1() {
    let mut m = MyBox::new(String::from("Jack1"));

    hello(&mut m);
    hello(&mut m);

    mut_hello(&mut m);
    mut_hello(&mut (*m)[..]);
}

fn main() {
    deref_demo_1()
}
