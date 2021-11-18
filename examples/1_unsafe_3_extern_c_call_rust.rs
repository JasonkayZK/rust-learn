#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("just called a rust func from C!");
}

fn main() {
    println!("build func upper to dynamic lib to use in C")
}
