use std::ops::Add;

fn fn_closure() {
    let fn_closure = |x| {
        println!("unmut_closure demo");
        x
    };

    let str1 = String::from("hello");
    let str2 = fn_closure(str1);
    println!("after unmut_closure: {}\n", str2);
}

fn fn_mut_closure() {
    let fn_mut_closure = |x: String| {
        x.add(", world")
    };
    let y1 = String::from("Hello");

    // Error here: y1's value has been removed
    // fn_mut_closure(y1);
    // println!("after fn_mut_closure: {}", y1);

    let y1 = fn_mut_closure(y1);
    println!("after fn_mut_closure: {}\n", y1);
}

fn fn_once_closure() {
    let x = vec![1,2,3];

    // Force remove x's ownership here!
    let fn_once_closure = move |z| z == x;
    // println!("Can't use x here: {:?}", x);

    let y = vec![1,2,3];
    println!("x == y? {}", fn_once_closure(y));
}

fn main() {
    fn_closure();
    fn_mut_closure();
    fn_once_closure();
}
