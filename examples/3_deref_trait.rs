use std::ops::Deref;

fn refer_demo_1() {
    let x = 5;
    let p_y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);

    // Error: no implementation for `{integer} == &{integer}`
    // assert_eq!(5, p_y);
    assert_eq!(5, *p_y);

    assert_eq!(5, *y);
}

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

fn refer_demo_2() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main() {
    refer_demo_1();
    refer_demo_2();
}
