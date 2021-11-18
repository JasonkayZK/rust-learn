// Immutable static var
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn immutable_static_var() {
    println!("name is: {}", HELLO_WORLD);
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mutable_static_var() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    immutable_static_var();

    mutable_static_var();
}
