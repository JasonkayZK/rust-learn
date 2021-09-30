use std::ops::Deref;

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

fn hello(name: &str) {
    println!("hello, {}", name);
}

fn deref_demo_1() {
    let m = MyBox::new(String::from("Jack1"));
    hello(&m);
    hello(&(m.deref()));
    hello(&(m.deref().deref()));
    hello(&(*m)[..]);
}

fn main() {
    deref_demo_1()
}
