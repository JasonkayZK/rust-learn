use std::sync::{Mutex, MutexGuard};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let lock = Mutex::new(2);
        increment_and_do_stuff(&lock).await;
    })
    .await
    .unwrap();
}

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    }

    // Won't work right now:
    // let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    // *lock += 1;
    // drop(lock);

    do_something_async().await;
} // lock goes out of scope here

async fn do_something_async() {
    println!("hello")
}
