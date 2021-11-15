fn create_raw_pointer_by_var() {
    let mut num = 5;

    // No `unsafe` to create raw pointer, but deref raw pointer should be unsafe!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("&r1: {:?}, &r2: {:?}", r1, r2);
}

fn create_raw_pointer_by_addr() {
    let addr = 0x012345usize;
    let r = addr as *const i32;

    println!("&r: {:?}", r);
}

fn unsafe_to_deref() {
    let mut num = 5;

    // No `unsafe` to create raw pointer, but deref raw pointer should be unsafe!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // We can use both const & mut ref at the same time!
        println!("r1: {:?}, r2: {:?}", *r1, *r2);
    }
}

fn unsafe_to_deref_and_change() {
    let mut num = 5;

    // No `unsafe` to create raw pointer, but deref raw pointer should be unsafe!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // We can use both const & mut ref at the same time, and change both value!
        *r2 = 10;
        println!("r1: {:?}, r2: {:?}", *r1, *r2);
    }
}

fn main() {
    create_raw_pointer_by_var();

    create_raw_pointer_by_addr();

    unsafe_to_deref();

    unsafe_to_deref_and_change();
}
