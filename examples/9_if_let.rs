fn main() {
    if_let_demo();
}

fn if_let_demo() {
    let some_u8_val = Some(0u8);

    /*
    match some_u8_val {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }
     */

    // Use if let instead
    if let Some(3) = some_u8_val {
        println!("three");
    } else {
        println!("not three");
    }
}
