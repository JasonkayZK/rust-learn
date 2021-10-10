use std::sync::{Arc, Mutex};
use std::thread;

fn multi_mutex_demo() {
    // let counter = Mutex::new(0);
    // let counter = Rc::new(Mutex::new(0));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Error: use of moved value: `counter`: value has been moved by other thread!
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}

fn main() {
    multi_mutex_demo()
}
