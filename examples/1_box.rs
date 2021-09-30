fn store_demo() {
    let b = Box::new(5);
    println!("b = {}", b); // Used as a value on stack

    // point b & b's value 5 will both released here!
}

fn main() {
    store_demo()
}
