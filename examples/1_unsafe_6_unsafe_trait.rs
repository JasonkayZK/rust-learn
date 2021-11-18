unsafe trait AddRawPointer {
    fn add_raw_pointer(&self, p: *mut i32);
}

unsafe impl AddRawPointer for i32 {
    fn add_raw_pointer(&self, p: *mut i32) {
        unsafe {
            *p = *p + self;
        }
    }
}

fn main() {
    let adder: i32 = 1;

    let mut i: i32 = 5;

    let p1 = &mut i as *mut i32;
    let p2 = &mut i as *mut i32;
    adder.add_raw_pointer(p1);
    adder.add_raw_pointer(p2);

    println!("i after add: {}", i);
}
