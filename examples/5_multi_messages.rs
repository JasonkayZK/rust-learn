use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    // Block recv()
    for received in rx {
        println!("got: {}", received);
    }
}
