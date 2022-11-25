use my_macro::elapsed;
use std::thread;
use std::time::Duration;

#[elapsed]
fn demo(t: u64) {
    let secs = Duration::from_secs(t);
    thread::sleep(secs);
}

fn main() {
    demo(4);
    demo(2);
}
