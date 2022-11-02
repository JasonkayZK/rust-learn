use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let lock = Mutex::new(2);
        increment_and_do_stuff(&CanIncrement { mutex: lock }).await;
    })
    .await
    .unwrap();
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

async fn do_something_async() {
    println!("hello")
}
