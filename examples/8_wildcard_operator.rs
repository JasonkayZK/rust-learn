fn main() {
    wildcard_demo();
}

fn wildcard_demo() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // All conditions!
        _ => (),
    }

}
