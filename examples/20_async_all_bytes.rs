use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut file = File::create("bar.txt").await.unwrap();

    file.write_all(b"all bytes").await.unwrap();

    println!("Wrote all bytes of 'some bytes'.");
}
