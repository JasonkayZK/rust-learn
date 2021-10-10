use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (tx, rx) = channel();

    spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // Error: value borrowed here after move
        // println!("val is {}", val);
    });

    // No block try_recv()
    // let received = rx.try_recv().unwrap();

    // Block recv()
    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
