use std::sync::Mutex;

fn lock_demo_1() {
    let m = Mutex::new(5);

    {
        // query lock
        let mut num = m.lock().unwrap();
        *num = 6;

        // m's lock will be released dual to Deref implemented by Mutex!
    }

    println!("m = {:?}", m);
}

fn main() {
    lock_demo_1();
}
