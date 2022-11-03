use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("bar.txt").await.unwrap();

    io::copy(&mut reader, &mut file).await.unwrap();
}
