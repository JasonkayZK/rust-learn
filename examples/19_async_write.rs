use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut file = File::create("bar.txt").await.unwrap();

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await.unwrap();

    println!("Wrote the first {} bytes of 'some bytes'.", n);
}
