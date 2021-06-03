fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    // s's value has been moved into takes_ownership function
    // so this is invalid!
    // println!("{}", s);

    let x = 5;
    makes_copy(x);

    // s's value has been copied into makes_copy function
    // valid
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
