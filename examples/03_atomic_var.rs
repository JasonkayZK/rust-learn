use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let cnt = 10000;

    for _ in 0..cnt {
        thread::spawn(|| {
            REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
        });
    }

    thread::sleep(Duration::from_secs(2));

    println!("当前用户请求数{:?}", REQUEST_RECV);
    assert_eq!(REQUEST_RECV.load(Ordering::Relaxed), cnt);
}
