fn never_type_demo1() -> ! {
    // Compiling err:
    // return ()
    loop {
        println!("ok");
    }
}

fn main() {}
