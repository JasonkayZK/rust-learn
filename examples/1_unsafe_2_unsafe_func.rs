use std::slice;

unsafe fn danger_func() {
    println!("i am not safe, read doc before using me!")
}

fn split_slice(a_slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let length = a_slice.len();

    assert!(mid <= length);

    // Compiling err: Two mutable references occurred! [Even in the total different parts]
    // (&mut a_slice[..mid], &mut a_slice[mid..])

    let ptr = a_slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), length - mid),
        )
    }
}

fn main() {
    // Compiling err
    // danger_func();

    unsafe {
        danger_func();
    }

    let mut a_slice = [1, 2, 3, 4, 5];
    let (s1, s2) = split_slice(&mut a_slice[..], 3);
    println!("s1: {:?}, s2: {:?}", s1, s2);
}
