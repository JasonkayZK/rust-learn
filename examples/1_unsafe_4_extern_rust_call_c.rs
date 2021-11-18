extern "C" {
    fn abs(input: i32) -> i32;
}

fn extern_c_demo() {
    unsafe { println!("abs of -3 with C: {}", abs(-3)) }
}

fn main() {
    extern_c_demo();
}
