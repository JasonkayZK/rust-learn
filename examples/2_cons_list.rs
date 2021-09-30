use List::{Cons, Nil};


#[derive(Debug)]
enum List {
    // Compiling Error: recursive without indirection
    // Cons(i32, List),

    Cons(i32, Box<List>),
    Nil,
}

fn list_demo_1() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    println!("list: {:?}", list)
}

fn main() {
    list_demo_1()
}
