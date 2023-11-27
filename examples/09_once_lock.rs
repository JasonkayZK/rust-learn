use std::sync::OnceLock;

static WINNER: OnceLock<&str> = OnceLock::new();

fn main() {
    let winner = std::thread::scope(|s| {
        s.spawn(|| WINNER.set("thread"));
        std::thread::yield_now(); // give them a chance...
        WINNER.get_or_init(|| "main")
    });
    println!("{winner:?} wins!");
}
