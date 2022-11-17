extern crate core;

use std::panic;

fn main() {
    let res = panic::catch_unwind(|| {
        println!("not panic");
    });
    assert!(res.is_ok());

    let res = panic::catch_unwind(|| {
        panic!("panicked!");
    });
    assert!(res.is_err());

    println!("End of main()");
}
