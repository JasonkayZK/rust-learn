fn main() {
    string_from_demo()
}

fn string_from_demo() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // No push_str function:
    // "hello".push_str(", world!");

    // This will not pass compile
    // println!("{}", "hello" + ", world!");

    // Fix:
    println!("{}", "hello".to_owned() + &", world!");
}
