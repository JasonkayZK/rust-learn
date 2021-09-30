use std::rc::Rc;

use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Compiling Error: recursive without indirection
    // Cons(i32, List),

    Cons(i32, Rc<List>),
    Nil,
}

fn rc_demo_1() {
    let a = Rc::new(Cons(1,
                         Rc::new(Cons(2,
                                      Rc::new(Cons(3,
                                                   Rc::new(Nil)))))));

    // Not to use a.clone(), to avoid deep copy!
    // let b = Cons(3, a.clone());
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("b: {:?}, c: {:?}", b, c);
}

fn rc_demo_2() {
    let a = Rc::new(Cons(5,
                         Rc::new(Cons(10,
                                      Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}


fn main() {
    rc_demo_1();
    rc_demo_2();
}
