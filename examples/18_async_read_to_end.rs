use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut f = File::open("foo.txt").await.unwrap();
    let mut buffer = vec![];

    // read the whole file
    f.read_to_end(&mut buffer).await.unwrap();
    println!("The bytes were: {:?}", buffer);
}
