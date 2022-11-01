#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value in spawned async"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
