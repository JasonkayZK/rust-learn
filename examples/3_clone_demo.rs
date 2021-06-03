fn main() {
    move_demo();
}

fn move_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Valid: s1's value has been copied!
    println!("{}, world", s1);

    // Valid:
    println!("{}, world", s2);
}
